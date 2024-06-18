use bevy::prelude::*;

pub mod assets;
pub mod controls;
pub mod kenney_assets;
pub mod movement;
pub mod pause;
pub mod player;

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