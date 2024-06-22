use bevy::prelude::*;

pub mod assets;
pub mod colors;
pub mod controls;
pub mod kenney_assets;
pub mod movement;
pub mod player;
pub mod settings;
pub mod ui;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States,)]
pub enum GameState {
    #[default]
    AssetLoading,
    Menu,
    ChooseShip,
    Playing,
}

#[derive(Component)]
pub struct Player;