use bevy::prelude::*;

use bevy_xpbd_2d::{plugins::{PhysicsDebugPlugin, PhysicsPlugins}};
use binding_of_isaac::{assets::AssetsPlugin, controls::ControlsPlugin, movement::MovementPlugin, player::PlayerPlugin, GameState};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0., 0., 0.1,
        )))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Binding of Isaac".into(),
                        ..default()
                    }),
                    ..default()
                }),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            (
                AssetsPlugin,
                ControlsPlugin,
                MovementPlugin,
                PlayerPlugin,
            )
            ))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}