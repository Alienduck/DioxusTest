use bevy::{
    ecs::relationship::Relationship,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
};
use chacha20::ChaCha8Rng;
use rand::{RngExt, SeedableRng};
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
                })
                .set(bevy::render::RenderPlugin {
                    render_creation: bevy::render::settings::RenderCreation::Automatic(Box::new(
                        bevy::render::settings::WgpuSettings {
                            backends: Some(
                                bevy::render::settings::Backends::BROWSER_WEBGPU
                                    | bevy::render::settings::Backends::GL,
                            ),
                            ..default()
                        },
                    )),
                    ..default()
                }),
        )
        .add_plugins(MaterialPlugin::<StarMaterial>::default())
        .insert_resource(ClearColor(Color::srgb(0.05, 0.02, 0.1)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                manage_startup,
                levitate,
                prepare_moon_material,
                animate_awakening,
            ),
        )
        .run();
}

#[derive(Resource)]
#[allow(unused)]
struct RandomSource(ChaCha8Rng);

#[derive(Component)]
struct Moon;

#[derive(Component)]
struct EmissiveMaterial(f32);

#[derive(Component)]
struct AwakenAnimation {
    target_intensity: f32,
    duration: f32,
    timer: f32,
}

#[derive(Component)]
struct Starfield;

#[derive(Resource)]
struct StartupSequence {
    scene_handle: Handle<WorldAsset>,
    warmup_timer: Timer,
    fading: bool,
}

#[derive(Component)]
struct FadeScreen;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct StarMaterial {
    #[uniform(0)]
    settings: StarSettings,
}

#[derive(ShaderType, Debug, Clone)]
struct StarSettings {
    color: LinearRgba,
    intensity: f32,
    phase: f32,
    speed: f32,
}

impl Material for StarMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/star.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut star_materials: ResMut<Assets<StarMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection::default()),
        Transform::from_xyz(0.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        bevy::camera::Hdr,
        bevy::post_process::bloom::Bloom::NATURAL,
        // DistanceFog {
        //     color: Color::srgb(0.5, 0.2, 0.1),
        //     falloff: FogFalloff::Exponential { density: 0.15 },
        //     ..default()
        // },
        bevy::light::VolumetricFog {
            ambient_color: Color::srgb(0.5, 0.1, 0.4),
            ambient_intensity: 0.5,
            ..default()
        },
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

    let scene = asset_server.load("model3d/scene.gltf#Scene0");

    commands.insert_resource(StartupSequence {
        scene_handle: scene.clone(),
        warmup_timer: Timer::from_seconds(3.4, TimerMode::Once),
        fading: false,
    });

    commands.spawn((
        WorldAssetRoot(scene),
        Transform::from_xyz(0.5, 0.0, 0.0),
        Moon,
        EmissiveMaterial(10.0),
    ));

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::BLACK),
        FadeScreen,
    ));

    let mut rng = ChaCha8Rng::seed_from_u64(123456789);
    const COLORS: [LinearRgba; 3] = [
        LinearRgba::rgb(1.0, 0.8, 0.05),
        LinearRgba::rgb(0.1, 0.1, 0.6),
        LinearRgba::rgb(0.3, 0.1, 0.5),
    ];

    commands
        .spawn((Transform::default(), Starfield))
        .with_children(|parent| {
            for _ in 0..150 {
                let star_mesh = meshes.add(Sphere::new(rng.random_range(0.005..0.03)));
                let r = rng.random_range(1.5..4.0);
                let theta = rng.random_range(0.0..std::f32::consts::TAU);
                let phi = rng.random_range(0.0..std::f32::consts::PI);

                let x = r * phi.sin() * theta.cos();
                let y = r * phi.sin() * theta.sin();
                let z = r * phi.cos();

                let base_intensity = rng.random_range(10.0..=30.0);
                let phase = rng.random_range(0.0..std::f32::consts::TAU);
                let speed = rng.random_range(0.5..2.0);
                let base_color = COLORS[rng.random_range(0..COLORS.len())];

                let star_material = star_materials.add(StarMaterial {
                    settings: StarSettings {
                        color: base_color,
                        intensity: base_intensity,
                        phase,
                        speed,
                    },
                });

                parent.spawn((
                    Mesh3d(star_mesh.clone()),
                    MeshMaterial3d(star_material),
                    Transform::from_xyz(x, y, z),
                ));
            }
        });

    commands.insert_resource(RandomSource(rng));
}

fn manage_startup(
    mut commands: Commands,
    mut sequence: ResMut<StartupSequence>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut fade_query: Query<(Entity, &mut BackgroundColor), With<FadeScreen>>,
) {
    if !sequence.fading {
        if let Some(bevy::asset::LoadState::Loaded) =
            asset_server.get_load_state(&sequence.scene_handle)
        {
            sequence.warmup_timer.tick(time.delta());
            if sequence.warmup_timer.is_finished() {
                sequence.fading = true;
            }
        }
        return;
    }

    if let Ok((entity, mut bg_color)) = fade_query.single_mut() {
        let current_alpha = bg_color.0.alpha();
        let new_alpha = (current_alpha - time.delta_secs() * 1.5).max(0.0);

        *bg_color = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, new_alpha));

        if new_alpha == 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn levitate(
    mut moon_query: Query<&mut Transform, (With<Moon>, Without<Starfield>)>,
    mut starfield_query: Query<&mut Transform, (With<Starfield>, Without<Moon>)>,
    time: Res<Time>,
) {
    let elapsed = time.elapsed_secs();

    for mut transform in moon_query.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            std::f32::consts::FRAC_PI_4,
            (elapsed * 2.0).sin() * 0.1,
            (elapsed * 2.0).sin() * 0.1,
        );
        transform.translation.y = (elapsed * 1.5).sin() * 0.05;
    }

    for mut transform in starfield_query.iter_mut() {
        transform.rotation = Quat::from_rotation_y(elapsed * 0.03);
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
