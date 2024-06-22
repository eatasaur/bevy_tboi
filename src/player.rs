use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::{
    assets::ImageAssets,
    controls::MovementFactor,
    kenney_assets::KenneySpriteSheetAsset,
    movement::WrappingMovement,
    ui::pause::Pausable,
    Player, GameState
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerCharacterType::Isaac)
            .add_systems(
                PostUpdate,
                player_destroyed_event_handler
                    .run_if(resource_equals(
                        Pausable::NotPaused,
                    ))
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                PostUpdate,
                spawn_player_after_player_destroyed
                    .run_if(in_state(GameState::Playing)),
                // if lives have changed and is not 0
                // .run_if(resource_changed::<Lives>)
                // .run_if(not(resource_equals(Lives(0))))
                // if ship was just destroyed
                // .run_if(on_event::<RemoveLifeEvent>()),
            )
            .add_event::<PlayerDestroyed>();
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite_bundle: SpriteBundle,
    pub texture_atlas: TextureAtlas,
    pub player: Player,
    pub character_type: PlayerCharacterType,
    pub collider: Collider,
    pub wrapping_movement: WrappingMovement,
}

#[derive(Event)]
pub struct PlayerDestroyed {
    pub destroyed_at: Transform,
    pub character_type: PlayerCharacterType,
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
            PlayerCharacterType::Isaac => 1,
            PlayerCharacterType::Judas => 2,
            PlayerCharacterType::Eden => 3,
        }
    }
    pub fn life_atlas_index(&self) -> usize {
        match &self {
            PlayerCharacterType::Isaac => 1,
            PlayerCharacterType::Judas => 2,
            PlayerCharacterType::Eden => 3,
        }
    }
    pub fn all_players() -> Vec<PlayerCharacterType> {
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

fn player_destroyed_event_handler(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut events: EventReader<PlayerDestroyed>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
    mut effect: Query<(
        &mut Transform,
    )>,
    mut ship_movement: ResMut<MovementFactor>,
) {
    let Some(space_sheet) = sheets.get(&images.male_person_sheet)
    else {
        warn!("player_ship_destroyed_event_handler requires meteor sprites to be loaded");
        return;
    };

    let Ok((
        mut effect_transform,
    )) = effect.get_single_mut()
    else {
        warn!("effect not ready yet, returning");
        return;
    };

    for PlayerDestroyed {
        destroyed_at,
        character_type,
    } in &mut events.read()
    {
        effect_transform.translation =
            destroyed_at.translation;

        ship_movement.0 = Vec2::ZERO;
    }
}

fn spawn_player_after_player_destroyed(
    mut commands: Commands,
    images: Res<ImageAssets>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
    player_ship_type: Res<PlayerCharacterType>,
) {
    // if !lives.is_changed() || lives.0 == 0 || lives.0 == 3 {
    //     return;
    // }
    let Some(space_sheet) = sheets.get(&images.male_person_sheet)
    else {
        warn!("player_ship_destroyed_event_handler requires meteor sprites to be loaded");
        return;
    };

    // let engine_fire = commands
    //     .spawn((
    //         SpriteBundle {
    //             transform: Transform::from_xyz(
    //                 0., -60., 1.,
    //             ),
    //             texture: space_sheet.sheet.clone(),
    //             sprite: Sprite {
    //                 flip_y: true,
    //                 ..default()
    //             },
    //             visibility: Visibility::Hidden,
    //             ..default()
    //         },
    //         TextureAtlas {
    //             index: 74,
    //             layout: space_sheet
    //                 .texture_atlas_layout
    //                 .clone(),
    //         },
    //         PlayerEngineFire,
    //     ))
    //     .id();

    commands
        .spawn(PlayerBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(0., 0., 1.),
                texture: space_sheet.sheet.clone(),
                ..default()
            },
            texture_atlas: TextureAtlas {
                index: player_ship_type.base_atlas_index(),
                layout: space_sheet
                    .texture_atlas_layout
                    .clone(),
            },
            player: Player,
            character_type: player_ship_type.clone(),
            collider: player_ship_type.collider(),
            wrapping_movement: WrappingMovement,
        });
}