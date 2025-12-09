use bevy::{
    asset::LoadState,
    gltf::{Gltf, GltfMesh},
    prelude::*,
    mesh::{Indices, VertexAttributeValues},
};
use bevy_rapier3d::{
    math::Vect,
    prelude::{ActiveEvents, AdditionalMassProperties, Collider, CollidingEntities, ExternalForce, GravityScale, RigidBody, Velocity},
};

use crate::{assets::SceneResource, car::{Car, StuckTimer, CAR_START_POSITION}, input, movement::CarMovements};

pub fn setup(
    asset_server: Res<AssetServer>,
    assets_gltf: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
    scene_assets: ResMut<SceneResource>,
    mut commands: Commands,
    car_query: Query<(), With<Car>>,
) {
    // Prevent duplicate spawning when re-entering Running state (e.g., from GameOver -> Running)
    if !car_query.is_empty() {
        bevy::log::info!("Car already exists, skipping scene spawn");
        return;
    }
    
    bevy::log::info!("Scene setup called");
    if let LoadState::Loaded = asset_server.load_state(&scene_assets.handle)
        && let Some(scenes_gltf) = assets_gltf.get(&scene_assets.handle) {
            bevy::log::info!("Assets loaded, spawning car and terrain");
            let initial_transform = Transform {
                translation: CAR_START_POSITION,
                rotation: Quat::IDENTITY, // Ensure car starts with correct orientation
                scale: Vec3::ONE,
            };

            commands
                .spawn((
                    SceneRoot(scenes_gltf.named_scenes["CAR"].clone()),
                    initial_transform,
                    RigidBody::Dynamic,
                    Collider::round_cuboid(3.8, 1.4, 2.2, 0.3),
                        CollidingEntities::default(), // Track collisions to detect ground contact
                        AdditionalMassProperties::Mass(50.0), // Make car heavier but still movable
                        Velocity::zero(),
                        ExternalForce::default(),
                        GravityScale(20.0), // Very high for fast falling 
                        CarMovements::default(),
                        StuckTimer::default(), // Track if car is stuck
                        input::default_input_map(),
                        Car {
                        thrust: Vec3::new(5000.0, 2500.0, 2500.0), // Increased thrust to move heavier car
                        drag: Vec3::new(100.0, 2.0, 100.0), // Minimal Y drag for very fast falling
                    },
                ));

            // Spawn terrain scene - it will spawn even if we can't extract mesh data for collider
            let terrain_scene_handle = scenes_gltf.named_scenes.get("DESERT");
            
            if let Some(terrain_scene_handle) = terrain_scene_handle {
                let mut desert_entity = commands.spawn(SceneRoot(terrain_scene_handle.clone()));
                
                // Terrain needs to be a static rigid body for collisions to work
                desert_entity.insert(RigidBody::Fixed);
                
                bevy::log::info!("Desert terrain scene spawned");
                
                // Try to add collider from the solid terrain mesh (not the wireframe)
                // The wireframe mesh will render for visual effect, but collision uses the solid mesh
                let desert_mesh_handle = scenes_gltf.named_meshes.get("DESERT");
                
                if let Some(desert_mesh_handle) = desert_mesh_handle {
                    bevy::log::info!("Found DESERT mesh handle (solid terrain for collision)");
                    
                    let desert_mesh: Option<&Mesh> = gltf_meshes
                        .get(desert_mesh_handle)
                        .and_then(|gltf_mesh| {
                            bevy::log::info!("GltfMesh has {} primitives", gltf_mesh.primitives.len());
                            gltf_mesh.primitives.first()
                        })
                        .and_then(|desert_primitive| {
                            bevy::log::info!("Trying to get mesh from primitive: {:?}", desert_primitive.mesh);
                            meshes.get(&desert_primitive.mesh)
                        });

                    if let Some(desert_mesh) = desert_mesh {
                        bevy::log::info!("Successfully got desert mesh");
                        let attribute_positions = desert_mesh.attribute(Mesh::ATTRIBUTE_POSITION);
                        let indices = desert_mesh.indices();

                        // Match on both attribute and index types (like limbo_pass)
                        match (attribute_positions, indices) {
                            (
                                Some(VertexAttributeValues::Float32x3(vertex_values)),
                                Some(Indices::U32(index_values)),
                            ) => {
                                let vertices: Vec<Vect> = vertex_values
                                    .iter()
                                    .map(|v| Vec3::new(v[0], v[1], v[2]))
                                    .collect();

                                let indices: Vec<[u32; 3]> = index_values
                                    .chunks(3)
                                    .map(|chunk| [chunk[0], chunk[1], chunk[2]])
                                    .collect();

                                let vertex_count = vertices.len();
                                let triangle_count = indices.len();
                                if let Ok(collider) = Collider::trimesh(vertices, indices) {
                                    desert_entity.insert(collider);
                                    desert_entity.insert(ActiveEvents::COLLISION_EVENTS);
                                    bevy::log::info!("Desert collider added successfully with {} vertices, {} triangles", vertex_count, triangle_count);
                                } else {
                                    bevy::log::warn!("Failed to create desert trimesh collider");
                                }
                            }
                            (
                                Some(VertexAttributeValues::Float32x3(vertex_values)),
                                Some(Indices::U16(index_values)),
                            ) => {
                                // Handle U16 indices
                                let vertices: Vec<Vect> = vertex_values
                                    .iter()
                                    .map(|v| Vec3::new(v[0], v[1], v[2]))
                                    .collect();

                                let indices: Vec<[u32; 3]> = index_values
                                    .chunks(3)
                                    .map(|chunk| [chunk[0] as u32, chunk[1] as u32, chunk[2] as u32])
                                    .collect();

                                let vertex_count = vertices.len();
                                let triangle_count = indices.len();
                                if let Ok(collider) = Collider::trimesh(vertices, indices) {
                                    desert_entity.insert(collider);
                                    desert_entity.insert(ActiveEvents::COLLISION_EVENTS);
                                    bevy::log::info!("Desert collider added successfully with {} vertices, {} triangles (U16 indices)", vertex_count, triangle_count);
                                } else {
                                    bevy::log::warn!("Failed to create desert trimesh collider");
                                }
                            }
                            (pos_attr, idx) => {
                                bevy::log::warn!("Failed to extract desert mesh data for collider - unsupported format");
                                if pos_attr.is_none() {
                                    bevy::log::warn!("  - No position attribute found");
                                } else {
                                    bevy::log::warn!("  - Position attribute is not Float32x3");
                                }
                                if idx.is_none() {
                                    bevy::log::warn!("  - No indices found");
                                } else {
                                    bevy::log::warn!("  - Indices are not U32 or U16");
                                }
                            }
                        }
                    } else {
                        bevy::log::warn!("Failed to get desert mesh from GltfMesh or Mesh assets");
                    }
                } else {
                    bevy::log::warn!("DESERT mesh not found in named_meshes");
                }
            } else {
                bevy::log::error!("DESERT scene not found in named_scenes");
            }
    }
}
