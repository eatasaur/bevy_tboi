use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    kenney_assets::{
        KenneyAssetPlugin, KenneySpriteSheetAsset,
    },
    GameState,
};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(KenneyAssetPlugin)
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::Menu)
                    .load_collection::<ImageAssets>()
                    // .load_collection::<AudioAssets>()
                    .load_collection::<FontAssets>(),
            );
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "OpenSans-Regular.ttf")]
    pub open_sans_regular: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "grey_box.png")]
    pub box_unchecked: Handle<Image>,
    #[asset(path = "grey_boxCheckmark.png")]
    pub box_checked: Handle<Image>,
    #[asset(path = "glass_panel.png")]
    pub panel_glass: Handle<Image>,
    #[asset(path = "pattern_blueprint.png")]
    pub pattern_blueprint: Handle<Image>,
    #[asset(path = "Male person/Tilesheet/character_malePerson_sheetHD.xml")]
    pub male_person_sheet: Handle<KenneySpriteSheetAsset>,
    #[asset(path = "Space/space_sheet.xml")]
    pub space_sheet: Handle<KenneySpriteSheetAsset>,
    
}
