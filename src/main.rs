use bevy::{
    prelude::*,
    window::{MonitorSelection, Window, WindowMode, WindowPlugin, WindowResolution},
};

#[derive(Component)]
struct SpinningCube;

#[derive(Component)]
struct OptionsMenu;

#[derive(Component)]
struct WindowModeButton;

#[derive(Resource)]
struct MenuVisible(bool);

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
        .insert_resource(MenuVisible(false))
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_fullscreen, handle_exit, rotate_cube, toggle_menu, handle_menu_interaction)) // Runs every frame
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

fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            OptionsMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(400.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    BorderRadius::all(Val::Px(10.0)),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Options"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // Window Mode Button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
                            BorderRadius::all(Val::Px(5.0)),
                            WindowModeButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Toggle Window Mode"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // Close Button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(100.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.6, 0.3, 0.3)),
                            BorderRadius::all(Val::Px(5.0)),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Close"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
}

fn toggle_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut menu_visible: ResMut<MenuVisible>,
    mut commands: Commands,
    menu_query: Query<Entity, With<OptionsMenu>>,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        menu_visible.0 = !menu_visible.0;
        
        if menu_visible.0 {
            setup_menu(commands);
        } else {
            // Despawn existing menu
            for entity in menu_query.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn handle_menu_interaction(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, Option<&WindowModeButton>), (Changed<Interaction>, With<Button>)>,
    mut windows: Query<&mut Window>,
    mut menu_visible: ResMut<MenuVisible>,
    mut commands: Commands,
    menu_query: Query<Entity, With<OptionsMenu>>,
) {
    for (interaction, mut color, window_mode_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if window_mode_button.is_some() {
                    // Handle window mode toggle
                    if let Ok(mut window) = windows.single_mut() {
                        window.mode = match window.mode {
                            WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                            WindowMode::BorderlessFullscreen(_) => WindowMode::Windowed,
                            WindowMode::Fullscreen(_, _) => WindowMode::Windowed,
                        };
                    }
                } else {
                    // Close button pressed
                    menu_visible.0 = false;
                    for entity in menu_query.iter() {
                        commands.entity(entity).despawn();
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.6, 0.6, 0.6).into();
            }
            Interaction::None => {
                if window_mode_button.is_some() {
                    *color = Color::srgb(0.4, 0.4, 0.4).into();
                } else {
                    *color = Color::srgb(0.6, 0.3, 0.3).into();
                }
            }
        }
    }
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
