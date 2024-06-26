use assets::ImageAssets;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;
use controls::{Laser, PlayerOwned};
use kenney_assets::KenneySpriteSheetAsset;
// use levels::Level;
use lives::Lives;
// use meteors::{
//     Meteor, MeteorBundle, MeteorDestroyed, MeteorType,
// };
use movement::WrappingMovement;
use rand::Rng;
// use scores::Scores;
use player::{
    PlayerCharacterType, PlayerBundle,
    PlayerDestroyed,
};
// use ufo::{Ufo, UfoDestroyed, UfoOwned};
use ui::choose_ship::ChoosePlayerEvent;

pub mod assets;
pub mod colors;
pub mod controls;
pub mod kenney_assets;
pub mod lives;
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

pub fn start_game(
    mut commands: Commands,
    images: Res<ImageAssets>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
    mut player_character_type_choice: ResMut<PlayerCharacterType>,
    // player_ship_type: Res<PlayerShipType>,
    // where the ship should spawn from before landing at
    // 0,0
    // spawn_from: Res<SpawnFrom>,
    mut choose_ship_reader: EventReader<
        ui::choose_ship::ChoosePlayerEvent,
    >,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else {
        warn!("no primary window, can't start game");
        return;
    };

    let Some(ChoosePlayerEvent {
        character_type,
        player_menu_location,
    }) = choose_ship_reader.read().next()
    else {
        warn!("No ChooseShipEvent coming from the menu; Check to make sure events are receivable.");
        return;
    };
    *player_character_type_choice = character_type.clone();

    let space_sheet =
        sheets.get(&images.space_sheet).unwrap();

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
                // transform: Transform::from_xyz(0., 0.,
                // 1.),
                transform: *player_menu_location,
                texture: space_sheet.sheet.clone(),
                ..default()
            },
            texture_atlas: TextureAtlas {
                index: character_type.base_atlas_index(),
                layout: space_sheet
                    .texture_atlas_layout
                    .clone(),
            },
            player: Player,
            character_type: character_type.clone(),
            collider: character_type.collider(),
            wrapping_movement: WrappingMovement,
        });
        // .add_child(engine_fire);

    // let width = window.resolution.width() / 2.;
    // let height = window.resolution.height() / 2.;

    // let mut rng = rand::thread_rng();
    // // TODO: spawn meteors according to current Level
    // // TODO: Make sure meteors don't spawn on ships
    // commands.spawn(MeteorBundle::big(
    //     Transform::from_xyz(
    //         rng.gen_range(-width..width),
    //         rng.gen_range(-height..height),
    //         1.,
    //     ),
    //     space_sheet,
    // ));
}