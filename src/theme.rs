use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioSource, AudioTween};

#[derive(Resource)]
pub struct ThemeState {
    pub loop_handle: Handle<AudioSource>,
    pub instance: Option<Handle<AudioInstance>>,
    pub is_playing: bool,
}

#[derive(Component)]
pub struct MusicToggleButton;

#[derive(Component)]
pub struct MusicToggleText;

type ButtonInteractionQuery<'w, 's> = Query<'w, 's, (&'static Interaction, &'static Children), (Changed<Interaction>, With<Button>)>;

pub fn load(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let loop_handle = asset_server.load("audio/overworld-lofi-random-halfspeed.ogg");
    let theme_state = ThemeState {
        loop_handle,
        instance: None,
        is_playing: false,
    };

    commands.insert_resource(theme_state);
}

pub fn play(mut audio_state: ResMut<ThemeState>, audio: Res<Audio>) {
    if audio_state.instance.is_none() {
        let instance = audio.play(audio_state.loop_handle.clone()).looped().handle();
        audio_state.instance = Some(instance);
        audio_state.is_playing = true;
        bevy::log::info!("Music started");
    }
}

pub fn spawn_music_toggle(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("font/NotoSansMono-Bold.ttf");
    
    commands
        .spawn((
            Node {
                width: Val::Px(120.),
                height: Val::Px(40.),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                right: Val::Px(10.),
                top: Val::Px(10.),
                ..default()
            },
            MusicToggleButton,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    BackgroundColor(Color::NONE),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text("music".to_string()),
                        TextFont {
                            font: font_handle,
                            font_size: 16.,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        MusicToggleText,
                    ));
                });
        });
}

pub fn handle_music_toggle(
    mut interaction_query: ButtonInteractionQuery,
    mut text_color_query: Query<&mut TextColor, With<MusicToggleText>>,
    mut audio_state: ResMut<ThemeState>,
    audio: Res<Audio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    for (interaction, children) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // Check if this button contains the music toggle text
            if let Some(child) = children.first().copied()
                && text_color_query.get(child).is_ok()
            {
                // Toggle music
                if audio_state.is_playing {
                    // Pause the music
                    if let Some(instance_handle) = &audio_state.instance
                        && let Some(instance) = audio_instances.get_mut(instance_handle)
                    {
                        instance.pause(AudioTween::default());
                        audio_state.is_playing = false;
                        bevy::log::info!("Music paused");
                    }
                } else {
                    // Resume or restart the music
                    if let Some(instance_handle) = &audio_state.instance {
                        if let Some(instance) = audio_instances.get_mut(instance_handle) {
                            instance.resume(AudioTween::default());
                            audio_state.is_playing = true;
                            bevy::log::info!("Music resumed");
                        } else {
                            // Instance was removed, restart the audio
                            let instance = audio.play(audio_state.loop_handle.clone()).looped().handle();
                            audio_state.instance = Some(instance);
                            audio_state.is_playing = true;
                            bevy::log::info!("Music restarted");
                        }
                    } else {
                        // No instance, start the audio
                        let instance = audio.play(audio_state.loop_handle.clone()).looped().handle();
                        audio_state.instance = Some(instance);
                        audio_state.is_playing = true;
                        bevy::log::info!("Music started");
                    }
                }
                
                // Update button text color to indicate state
                if let Ok(mut text_color) = text_color_query.get_mut(child) {
                    if audio_state.is_playing {
                        *text_color = TextColor(Color::srgb(0.9, 0.9, 0.9)); // Bright when on
                    } else {
                        *text_color = TextColor(Color::srgb(0.4, 0.4, 0.4)); // Dim when off
                    }
                }
            }
        }
    }
}

