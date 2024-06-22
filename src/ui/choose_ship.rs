use bevy::prelude::*;

use crate::{
    assets::ImageAssets,
    kenney_assets::KenneySpriteSheetAsset,
    settings::{AudioSettings, GameSettings},
    player::PlayerCharacterType,
    GameState,
};

pub struct ChoosePlayerPlugin;

impl Plugin for ChoosePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChoosePlayerEvent>()
            .add_systems(
                OnEnter(GameState::ChooseShip),
                choose_player_menu,
            )
            .add_systems(
                Update,
                choose_player_button_system.run_if(in_state(
                    GameState::ChooseShip,
                )),
            )
            .add_systems(
                OnExit(GameState::ChooseShip),
                hide_player_menu,
            );
    }
}

#[derive(Event)]
pub struct ChoosePlayerEvent {
    pub character_type: PlayerCharacterType,
    pub player_menu_location: Transform,
}

#[derive(Component)]
pub struct ChooseShipMenu;

#[derive(Debug, Component)]
pub struct ShipIndex(pub usize);

pub fn hide_player_menu(
    mut choose_ship_menu: Query<
        &mut Visibility,
        With<ChooseShipMenu>,
    >,
) {
    for mut visibility in &mut choose_ship_menu {
        *visibility = Visibility::Hidden;
    }
}
pub fn choose_player_menu(
    mut commands: Commands,
    images: Res<ImageAssets>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
    mut choose_ship_menu: Query<
        &mut Visibility,
        With<ChooseShipMenu>,
    >,
) {
    if !choose_ship_menu.is_empty() {
        let mut visibility = choose_ship_menu.single_mut();
        *visibility = Visibility::Visible;
        return;
    }
    let male_person_sheet =
        sheets.get(&images.male_person_sheet).unwrap();
    let players: Vec<_> = PlayerCharacterType::all_players()
        .into_iter()
        .map(|player_type| {
            let player = commands
                .spawn((
                    ImageBundle {
                        image: male_person_sheet
                            .sheet
                            .clone()
                            .into(),
                        ..default()
                    },
                    TextureAtlas {
                        index: player_type.base_atlas_index(),
                        layout: male_person_sheet
                            .texture_atlas_layout
                            .clone(),
                    },
                ))
                .id();
            commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(200.0),
                            justify_content:
                                JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        image: images
                            .pattern_blueprint
                            .clone()
                            .into(),
                        ..default()
                    },
                    ImageScaleMode::Tiled {
                        tile_x: true,
                        tile_y: true,
                        stretch_value: 0.5,
                    },
                    player_type,
                ))
                .add_child(player)
                .id()
        })
        .collect();

    let mut wrapper = commands.spawn((
        NodeBundle {
            style: Style {
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.),
                ..default()
            },
            ..default()
        },
        ChooseShipMenu,
    ));

    for player in players {
        wrapper.add_child(player);
    }
}

pub fn choose_player_button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &PlayerCharacterType,
            &Transform,
        ),
        Changed<Interaction>,
    >,
    settings: Res<GameSettings>,
    // sounds: Res<AudioAssets>,
    mut next_state: ResMut<NextState<GameState>>,
    mut choose_ship_events: EventWriter<ChoosePlayerEvent>,
) {
    for (interaction, character_type, transform) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                if settings.audio == AudioSettings::ON {
                    // commands.spawn(AudioBundle
                    // {
                    //     source:
                    // sounds.apple.clone(),
                    //     ..default()
                    // });
                }
                // *color = PRESSED_BUTTON.into();

                choose_ship_events.send(ChoosePlayerEvent {
                    character_type: character_type.clone(),
                    player_menu_location: transform.clone(),
                });
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                // if settings.audio == AudioSettings::ON {
                //     commands.spawn(AudioBundle {
                //         source: sounds.menu_click.clone(),
                //         ..default()
                //     });
                // }
                // *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                // *color = NORMAL_BUTTON.into();
            }
        }
    }
}