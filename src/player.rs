use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::{
    movement::WrappingMovement,
    Player,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {
        
    }
}

#[derive(Bundle)]
pub struct ShipBundle {
    pub sprite_bundle: SpriteBundle,
    pub texture_atlas: TextureAtlas,
    pub player: Player,
    pub character_type: PlayerCharacterType,
    pub collider: Collider,
    pub wrapping_movement: WrappingMovement,
}

#[derive(Resource, Component, Clone)]
pub enum PlayerCharacterType {
    Isaac,
    Judas,
    Eden,
}

impl PlayerCharacterType {
    pub fn base_atlas_index(&self) -> usize {
        match &self {
            PlayerCharacterType::Isaac => 200,
            PlayerCharacterType::Judas => 207,
            PlayerCharacterType::Eden => 214,
        }
    }
    pub fn life_atlas_index(&self) -> usize {
        match &self {
            PlayerCharacterType::Isaac => 188,
            PlayerCharacterType::Judas => 192,
            PlayerCharacterType::Eden => 196,
        }
    }
    pub fn all_ships() -> Vec<PlayerCharacterType> {
        vec![
            PlayerCharacterType::Isaac,
            PlayerCharacterType::Judas,
            PlayerCharacterType::Eden,
        ]
    }
    pub fn collider(&self) -> Collider {
        Collider::capsule(40., 10.)
    }
    pub fn base_ship_speed(&self) -> BaseCharacterSpeed {
        match self {
            PlayerCharacterType::Isaac => BaseCharacterSpeed {
                movement_speed: 500.0, // meters per second
                rotation_speed: f32::to_radians(360.0), /* degrees per second */
            },
            PlayerCharacterType::Judas => BaseCharacterSpeed {
                movement_speed: 500.0, // meters per second
                rotation_speed: f32::to_radians(360.0), /* degrees per second */
            },
            PlayerCharacterType::Eden => BaseCharacterSpeed {
                movement_speed: 500.0, // meters per second
                rotation_speed: f32::to_radians(360.0), /* degrees per second */
            },
        }
    }
}

pub struct BaseCharacterSpeed {
    /// linear speed in meters per second
    pub movement_speed: f32,
    /// rotation speed in radians per second
    pub rotation_speed: f32,
}