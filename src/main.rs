use bevy::{
    prelude::*,
    window::{MonitorSelection, Window, WindowMode, WindowPlugin, WindowResolution},
};

#[derive(Component)]
struct SpinningCube;

fn main() {
    print!("Hello, Bevy!\n");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fullscreen Toggle".to_string(),
                // TODO: Add option UI for common resolutions
                resolution: WindowResolution::new(800.0, 600.0),
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_fullscreen, handle_exit, rotate_cube)) // Runs every frame
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
     // Spawn a new Entity and attach a Camera3d Component
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Spawn a new Entity and attach a Mesh3d Component
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        SpinningCube,
    ));
    
    // Spawn a new Entity and attach a DirectionalLight Component
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 3000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::FRAC_PI_4,
        )),
    ));
}

fn rotate_cube(mut query: Query<&mut Transform, With<SpinningCube>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 1.0); // Rotate 1 radian per second
    }
}

fn toggle_fullscreen(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>, // Ask for all entities that have a Window Component
) {
    if keyboard_input.just_pressed(KeyCode::F11) {
        if let Ok(mut window) = windows.single_mut() {
            window.mode = match window.mode {
                WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                // TODO: Add option ui for modes Windowed, BorderlessFullscreen, Fullscreen
                _ => WindowMode::Windowed,
            };
        } else {
            warn!("Could not find primary window to toggle fullscreen.");
        }
    }
}

fn handle_exit(keyboard_input: Res<ButtonInput<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        EventWriter::write(&mut app_exit_events, AppExit::Success);
    }
}
