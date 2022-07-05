use bevy::{
    asset::LoadState,
    gltf::{Gltf, GltfMesh},
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};
use bevy_rapier3d::{
    math::Vect,
    plugin::RapierConfiguration,
    prelude::{ActiveEvents, Collider, ExternalForce, LockedAxes, RigidBody, Velocity},
};
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

use crate::{
    assets::SceneAssets,
    buggy::{Buggy, Movements},
    state::GameState,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Setup)
                .with_system(scene.label("scene"))
                .with_system(lighting.after("scene").label("lighting"))
                .with_system(physics.after("lighting").label("physics")),
        )
        .add_system_set(SystemSet::on_update(GameState::Setup).with_system(complete))
        .add_system_set(
            SystemSet::on_enter(GameState::Running).with_system(camera.label("camera")),
        );
    }
}

pub fn complete(
    mut state: ResMut<State<GameState>>,
) {
    state.set(GameState::Running).unwrap();
}

pub fn camera(mut commands: Commands) {
    commands.spawn_bundle(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(-100.0, 60.0, 20.0),
        Vec3::new(0.0, 0.0, 0.0),
    ));
}

pub fn physics(mut physics_config: ResMut<RapierConfiguration>) {
    physics_config.gravity = Vec3::ZERO;
    physics_config.gravity.y = -100.0;
}

pub fn lighting(mut commands: Commands) {
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

pub fn scene(
    asset_server: Res<AssetServer>,
    assets_gltf: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
    scene_assets: ResMut<SceneAssets>,
    mut commands: Commands,
) {
    if let LoadState::Loaded = asset_server.get_load_state(&scene_assets.handle) {
        if let Some(scenes_gltf) = assets_gltf.get(&scene_assets.handle) {
            commands
                .spawn_bundle(TransformBundle::from(Transform::from_xyz(-45.0, 1.5, 0.0)))
                .insert(RigidBody::Dynamic)
                .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
                .insert(Collider::ball(2.3))
                .insert(Velocity::zero())
                .insert(ExternalForce::default())
                .insert(Movements::default())
                .insert(Buggy {
                    // nothing special about these values, just played around until it felt like a ghost
                    thrust: Vec3::new(600.0, 100.0, 600.0),
                    drag: Vec3::new(250.0, 500.0, 250.0),
                })
                .with_children(|parent| {
                    parent.spawn_scene(scenes_gltf.named_scenes["BUGGY"].clone());
                });

            let terrain_mesh_handle = &scenes_gltf.named_meshes["TERRAIN"];

            let terrain_mesh: Option<&Mesh> = gltf_meshes
                .get(terrain_mesh_handle)
                .and_then(|gltf_mesh| gltf_mesh.primitives.get(0))
                .and_then(|terrain_primitive| meshes.get(&terrain_primitive.mesh));

            let attribute_positions =
                terrain_mesh.and_then(|m| m.attribute(Mesh::ATTRIBUTE_POSITION));

            if let (
                Some(VertexAttributeValues::Float32x3(vertex_values)),
                Some(Indices::U32(index_values)),
            ) = (attribute_positions, terrain_mesh.and_then(|m| m.indices()))
            {
                let vertices: Vec<Vect> = vertex_values
                    .iter()
                    .map(|v| Vec3::new(v[0], v[1], v[2]))
                    .collect();

                let indices: Vec<[u32; 3]> = index_values
                    .chunks(3)
                    .map(|chunk| [chunk[0], chunk[1], chunk[2]])
                    .collect();

                commands
                    .spawn()
                    .insert(Collider::trimesh(vertices, indices))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .with_children(|parent| {
                        parent.spawn_scene(scenes_gltf.named_scenes["TERRAIN"].clone());
                    });
            }
        }
    }
}