use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.50, 0.50, 0.50)))
        .insert_resource(WindowDescriptor {
            title: "Rpg".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    // camera
    commands.spawn_bundle(Camera2dBundle::default());
}