use bevy::prelude::*;

pub fn setup(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    // Update ambient light using ResMut
    ambient_light.brightness = 1000.0;
    ambient_light.color = Color::srgb(0.75, 0.75, 0.75); // Silver/gray tone (like limbo_pass)
    ambient_light.affects_lightmapped_meshes = true;
    
    // Add a directional light (moon) for overall scene illumination with shadows
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.9, 0.95, 1.0), // Cool moonlight color
            illuminance: 10000.0, // Bright enough to light the scene
            shadows_enabled: true, // Enable shadows
            shadow_depth_bias: 0.02,
            shadow_normal_bias: 0.8,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.3, 0.0)), // Angle the moon
    ));
}
