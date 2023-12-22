mod parser;
use bevy_rapier3d::prelude::*;
use bevy::{core::FrameCount, window::PresentMode, app::AppExit};
use parser::brick;
use bevy::prelude::*;
use itertools::Itertools;
use rand::{thread_rng, Rng};

#[derive(Component)]
struct LookAtTarget(Vec3);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Collider::cuboid(1000.0, 0.2, 1000.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.1, 0.0)));

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(100.0).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::GREEN.into(),
            perceptual_roughness: 1.0,
            ..default()
        }),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)).with_translation(Vec3::new(0., -2., 0.)),
        ..default()
    });

    let input = std::fs::read_to_string("../../input/day22.txt").unwrap();
    let bricks = input.lines()
        .map(|line| brick(line).unwrap().1);
    let mut rng = thread_rng();

    for b in bricks {
        let (x_length, y_length, z_length) = (0..3).map(|i| {
            (b.coords[i].1 - b.coords[i].0 + 1) as f32
        }).collect_tuple().unwrap();

        let (x, y, z) = (0..3).map(|i| {
            (b.coords[i].0 as f32 + b.coords[i].1 as f32 + 1.0) / 2.0
        }).collect_tuple().unwrap();

        commands.spawn(RigidBody::Dynamic)
            .insert(GravityScale(0.0))
            .insert(Collider::cuboid(x_length / 2.0, z_length / 2.0, y_length / 2.0))
            .insert(TransformBundle::from(Transform::from_xyz(x, z, y)))
            .with_children(|builder| {
                builder.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(x_length, z_length, y_length))),
                    material: materials.add(Color::hsl(rng.gen_range(0.0..360.0), rng.gen_range(0.9..1.0), rng.gen_range(0.4..0.6)).into()),
                    ..default()
                });
            });
    }

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(1.0, 248.0/255.0, 179.0/255.0),
            illuminance: 20_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(100.0, 100.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(1.0, 1.0, 220.0/255.0),
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-100.0, -100.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-30.5, 50.5, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(LookAtTarget(Vec3::ZERO));
}

fn camera_control(
    mut camera: Query<(&mut Transform, &mut LookAtTarget), With<Camera>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut camera, mut target) = camera.single_mut();

    let mut dy = 0.0;
    if input.pressed(KeyCode::Up) {
        dy += 1.0;
    }

    if input.pressed(KeyCode::Down) {
        dy -= 1.0;
    }

    if dy != 0.0 {
        target.0.y += dy * 30.0 * time.delta_seconds();
        camera.look_at(target.0, Vec3::Y);
    }
}

fn start_sim(
    mut query: Query<&mut GravityScale>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for mut scale in query.iter_mut() {
            scale.0 = 3.0;
        }
    }
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.single_mut().visible = true;
    }
}

fn exit_on_esc(input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "tetris?! maybe not...".into(),
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                present_mode: PresentMode::AutoVsync,
                // This will spawn an invisible window
                // The window will be made visible in the make_visible() system after 3 frames.
                // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                visible: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Update, make_visible)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_control)
        .add_systems(Update, start_sim)
        .add_systems(Update, exit_on_esc)
        .run()
}
