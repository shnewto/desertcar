use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 2.0,
    });
    let point_light_intensity = 60000.0;
    let point_light_range = 500.;
    let point_light_color_hex_string = "f0cf3c";
    let color = Color::hex(point_light_color_hex_string).unwrap_or_else(|_| {
        panic!(
            "couldn't make hex color from {}",
            point_light_color_hex_string
        )
    });
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color,
            range: point_light_range,
            intensity: point_light_intensity,
            ..Default::default()
        },
        transform: Transform::from_xyz(-40.0, 20.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color,
            range: point_light_range,
            intensity: point_light_intensity,
            ..Default::default()
        },
        transform: Transform::from_xyz(40.0, 20.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color,
            range: point_light_range,
            intensity: point_light_intensity,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 20.0, -40.0),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color,
            range: point_light_range,
            intensity: point_light_intensity,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 20.0, 40.0),
        ..Default::default()
    });
}
