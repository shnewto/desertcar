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

use crate::{assets::SceneAssets, car::Car, movement::CarMovements};

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
            let initial_transform = Transform::from_xyz(-700.0, 1.0, 0.0);

            commands
                .spawn_bundle(TransformBundle::from(initial_transform))
                .insert(RigidBody::Dynamic)
                .insert(Collider::round_cuboid(3.8, 1.4, 2.2, 0.3))
                .insert(Velocity::zero())
                .insert(ExternalForce::default())
                .insert(CarMovements::default())
                .insert(Car {                   
                    thrust: Vec3::new(2800.0, 1400.0, 1400.0),
                    drag: Vec3::new(100.0, 50.0, 100.0),
                })
                .with_children(|parent| {
                    parent.spawn_scene(scenes_gltf.named_scenes["CAR"].clone());
                });

            let desert_mesh_handle = &scenes_gltf.named_meshes["DESERT"];

            let desert_mesh: Option<&Mesh> = gltf_meshes
                .get(desert_mesh_handle)
                .and_then(|gltf_mesh| gltf_mesh.primitives.get(0))
                .and_then(|desert_primitive| meshes.get(&desert_primitive.mesh));

            let attribute_positions =
                desert_mesh.and_then(|m| m.attribute(Mesh::ATTRIBUTE_POSITION));

            if let (
                Some(VertexAttributeValues::Float32x3(vertex_values)),
                Some(Indices::U32(index_values)),
            ) = (attribute_positions, desert_mesh.and_then(|m| m.indices()))
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
                        parent.spawn_scene(scenes_gltf.named_scenes["DESERT"].clone());
                    });
            }
        }
    }
}
