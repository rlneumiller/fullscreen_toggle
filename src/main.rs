use bevy::{
    prelude::*,
    window::{MonitorSelection, Window, WindowMode, WindowPlugin, WindowResolution},
};

fn main() {
    print!("Hello, Bevy!\n");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fullscreen Toggle".to_string(),
                resolution: WindowResolution::new(800.0, 600.0),
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_fullscreen, handle_exit))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn toggle_fullscreen(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
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
