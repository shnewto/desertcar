use bevy::prelude::*;

pub fn setup(mut ambient_light: ResMut<AmbientLight>) {
    // Update ambient light using ResMut
    ambient_light.brightness = 2000.0; // Increased ambient light
    ambient_light.color = Color::srgb(0.75, 0.75, 0.75); // Silver/gray tone (like limbo_pass)
    ambient_light.affects_lightmapped_meshes = true;
}
