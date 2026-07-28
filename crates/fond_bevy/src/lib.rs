use bevy::{ecs::relationship::Relationship, prelude::*};
use chacha20::ChaCha8Rng;
use rand::SeedableRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#bevy-canvas".into()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.05)))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate, make_moon_emissive))
        .run();
}

#[derive(Resource)]
#[allow(unused)]
struct RandomSource(ChaCha8Rng);

#[derive(Component)]
struct Moon;

#[derive(Component)]
struct Rotate(f32);

#[derive(Component)]
struct EmissiveMaterial(f32);

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let seed = ChaCha8Rng::seed_from_u64(123456789);

    commands.insert_resource(RandomSource(seed));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        bevy::camera::Hdr,
        bevy::post_process::bloom::Bloom::ANAMORPHIC,
    ));

    commands.spawn((
        PointLight {
            intensity: 10000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        WorldAssetRoot(asset_server.load("model3d/scene.gltf#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            90.0,
            90.0,
            0.0,
        )),
        Moon,
        EmissiveMaterial(10.0),
        Rotate(0.5),
    ));
}

fn rotate(mut moon_query: Query<(&mut Transform, &Rotate)>, time: Res<Time>) {
    for (mut moon, rotation) in moon_query.iter_mut() {
        moon.rotate(Quat::from_euler(
            EulerRot::XYZ,
            rotation.0 * time.delta_secs(),
            rotation.0 * time.delta_secs(),
            rotation.0 * time.delta_secs(),
        ));
    }
}

fn make_moon_emissive(
    mut materials: ResMut<Assets<StandardMaterial>>,
    material_query: Query<
        (Entity, &MeshMaterial3d<StandardMaterial>),
        Added<MeshMaterial3d<StandardMaterial>>,
    >,
    parent_query: Query<&ChildOf>,
    emissive_query: Query<&EmissiveMaterial>,
) {
    for (entity, mesh_material) in material_query.iter() {
        let mut current_entity = entity;
        let mut target_intensity = None;

        loop {
            if let Ok(emissive) = emissive_query.get(current_entity) {
                target_intensity = Some(emissive.0);
                break;
            }
            if let Ok(parent) = parent_query.get(current_entity) {
                current_entity = parent.get();
            } else {
                break;
            }
        }

        if let Some(intensity) = target_intensity {
            if let Some(mut material) = materials.get_mut(mesh_material.0.id()) {
                material.emissive = material.base_color.to_linear() * intensity;
            }
        }
    }
}
