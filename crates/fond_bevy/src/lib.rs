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
        .add_systems(Update, (rotate, prepare_moon_material, animate_awakening))
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

#[derive(Component)]
struct AwakenAnimation {
    target_intensity: f32,
    duration: f32,
    timer: f32,
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let seed = ChaCha8Rng::seed_from_u64(123456789);
    commands.insert_resource(RandomSource(seed));

    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection::default()),
        Transform::from_xyz(0.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        bevy::camera::Hdr,
        bevy::post_process::bloom::Bloom::ANAMORPHIC,
    ));

    commands.spawn((
        PointLight {
            color: Color::srgb(0.6, 0.1, 0.9),
            intensity: 15000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 5.0, 4.0),
    ));

    commands.spawn((
        PointLight {
            color: Color::srgb(0.1, 0.4, 1.0),
            intensity: 10000.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(-4.0, -2.0, -4.0),
    ));

    commands.spawn((
        WorldAssetRoot(asset_server.load("model3d/scene.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            90.0,
            0.0,
            0.0,
        )),
        Moon,
        EmissiveMaterial(10.0),
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

fn prepare_moon_material(
    mut commands: Commands,
    material_query: Query<
        (Entity, &MeshMaterial3d<StandardMaterial>),
        Added<MeshMaterial3d<StandardMaterial>>,
    >,
    parent_query: Query<&ChildOf>,
    emissive_query: Query<&EmissiveMaterial>,
) {
    for (entity, _mesh_material) in material_query.iter() {
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
            commands.entity(entity).insert(AwakenAnimation {
                target_intensity: intensity,
                duration: 2.0,
                timer: 0.0,
            });
        }
    }
}

fn animate_awakening(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        Entity,
        &mut AwakenAnimation,
        &MeshMaterial3d<StandardMaterial>,
    )>,
) {
    for (entity, mut anim, mesh_material) in query.iter_mut() {
        anim.timer += time.delta_secs();

        let progress = (anim.timer / anim.duration).clamp(0.0, 1.0);

        let smooth_t = progress * progress * (3.0 - 2.0 * progress);

        let current_intensity = anim.target_intensity * smooth_t;

        if let Some(mut material) = materials.get_mut(mesh_material.0.id()) {
            let glow_color = Color::srgb(0.2, 0.5, 1.0).to_linear();
            material.emissive = glow_color * current_intensity;
        }

        if progress >= 1.0 {
            commands.entity(entity).remove::<AwakenAnimation>();
        }
    }
}
