use bevy::{
    asset::LoadState,
    gltf::{Gltf, GltfMesh},
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};
use bevy_rapier3d::{
    math::Vect,
    prelude::{ActiveEvents, Collider, ExternalForce, RigidBody, Velocity},
};

use crate::{
    assets::SceneAssets,
    buggy::{Buggy, Movements},
};

pub fn setup(
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
                .spawn_bundle(TransformBundle::from(Transform::from_xyz(-45.0, 0.0, 0.0)))
                .insert(RigidBody::Dynamic)
                // .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
                .insert(Collider::round_cuboid(3.0, 1.5, 1.0, 0.5))
                .insert(Velocity::zero())
                .insert(ExternalForce::default())
                .insert(Movements::default())
                .insert(Buggy {
                    thrust: Vec3::new(900.0, 500.0, 900.0),
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
