use bevy::prelude::*;

pub fn setup(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    // Update ambient light using ResMut
    ambient_light.brightness = 1000.0; // Increased ambient light
    ambient_light.color = Color::srgb(0.98, 0.92, 0.84); // Warm beige tone
    ambient_light.affects_lightmapped_meshes = true;
    let point_light_intensity = 10_000_000.0; // Much brighter like limbo_pass
    let point_light_range = 500.0;
    let color = Color::srgb_u8(0xf0, 0xcf, 0x3c);

    // Spawn 4 point lights around the scene (like limbo_pass pattern)
    for (x, z) in [(-40.0, 0.0), (40.0, 0.0), (0.0, -40.0), (0.0, 40.0)] {
        commands.spawn((
            PointLight {
                color,
                range: point_light_range,
                intensity: point_light_intensity,
                shadows_enabled: true, // Enable shadows like limbo_pass
                ..default()
            },
            Transform::from_xyz(x, 15.0, z),
        ));
    }
}
