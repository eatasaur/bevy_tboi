use bevy::prelude::*;

use bevy_xpbd_2d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};
use binding_of_isaac::{
    assets::AssetsPlugin, controls::ControlsPlugin, lives::LifePlugin,
    movement::MovementPlugin, 
    player::PlayerPlugin, settings::SettingsPlugin, 
    ui::{choose_ship::ChoosePlayerPlugin, pause::PausePlugin, UiPlugin}, 
    start_game, reset_game,
    GameState};

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
                ChoosePlayerPlugin,
                ControlsPlugin,
                LifePlugin,
                MovementPlugin,
                PausePlugin,
                PlayerPlugin,
                SettingsPlugin,
                UiPlugin,
            )
            ))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(
            OnEnter(GameState::Playing),
            (reset_game, start_game).chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}