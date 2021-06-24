use bevy::app::AppExit;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;
use orientation::Orientation;

mod orientation;

struct Camera;

struct Light;

struct Cube;

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
            ..Default::default()
        })
        .insert(Orientation::default());

    commands.spawn().insert(Light).insert_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    commands.spawn().insert(Cube).insert_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -5.0)),
        mesh: meshes.add(Mesh::from(shape::Cube::default())),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        ..Default::default()
    });
}

fn input(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut Orientation), With<Camera>>,
    mut exit: EventWriter<AppExit>,
) {
    let (mut transform, mut orientation) = query.single_mut().unwrap();

    let delta = time.delta_seconds() * MOUSE_SENSITIVITY;

    for event in motion.iter() {
        let delta_x = -event.delta.x * delta;
        let delta_y = -event.delta.y * delta;

        let rot_y = Quat::from_axis_angle(Vec3::Y, delta_x);
        orientation.rotate(rot_y);

        let rot_x = Quat::from_axis_angle(orientation.x, delta_y);
        orientation.rotate(rot_x);

        transform.rotation = Quat::from(*orientation);
    }

    let delta = time.delta_seconds() * MOVEMENT_SPEED;
    let z = orientation.z;
    let x = orientation.x;

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

    if keyboard.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
