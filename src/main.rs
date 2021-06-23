use bevy::app::AppExit;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;

struct Camera;

struct Light;

struct Icosphere;

const MOVEMENT_SPEED: f32 = 2.0;
const MOUSE_SENSITIVITY: f32 = 0.2;

fn main() {
    let mut app = App::build();
    app.insert_resource(WindowDescriptor {
        title: "ww".to_string(),
        width: 1920.0,
        height: 1080.0,
        cursor_locked: true,
        cursor_visible: false,
        ..Default::default()
    });
    app.insert_resource(Msaa { samples: 4 });
    app.insert_resource(Time::default());
    app.add_plugins(DefaultPlugins);
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.add_startup_system(setup.system());
    app.add_system(input.system());
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn()
        .insert(Camera)
        .insert_bundle(PerspectiveCameraBundle {
            perspective_projection: PerspectiveProjection {
                near: 0.01,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });

    commands.spawn().insert(Light).insert_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    commands.spawn().insert(Icosphere).insert_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        ..Default::default()
    });
}

fn input(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut motion: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }

    let transform = &mut query.single_mut().unwrap();
    let x = transform.local_x();
    let z = transform.local_z();
    let delta = time.delta_seconds() * MOVEMENT_SPEED;

    if keyboard.pressed(KeyCode::W) {
        transform.translation -= z * delta;
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation += z * delta;
    }
    if keyboard.pressed(KeyCode::A) {
        transform.translation -= x * delta;
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation += x * delta;
    }

    let delta = time.delta_seconds() * MOUSE_SENSITIVITY;

    for event in motion.iter() {
        transform.rotate(Quat::from_rotation_y(-event.delta.x * delta));
    }
}
