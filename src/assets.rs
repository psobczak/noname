use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::*;

use bevy_common_assets::ron::RonAssetPlugin;

use crate::GameState;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RonAssetPlugin::<AnimationsConfig>::new(&["animations.ron"]),))
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::Next)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                        "characters.assets.ron",
                    )
                    .load_collection::<ConfigHandles>()
                    .load_collection::<GameAssetsHandles>(),
            );
    }
}

#[derive(serde::Deserialize, Asset, TypePath, Debug)]
pub struct AnimationsConfig(pub HashMap<String, HashMap<String, Vec<usize>>>);

#[derive(AssetCollection, Resource, Asset, Reflect)]
pub struct GameAssetsHandles {
    #[asset(key = "characters.texture_atlas_layout")]
    pub characters_layouts: Handle<TextureAtlasLayout>,
    #[asset(key = "characters.sheets", collection(typed, mapped))]
    pub characters_sheets: HashMap<String, Handle<Image>>,
    #[asset(key = "characters.portrait")]
    pub portraits: Handle<Image>,
    #[asset(key = "monsters.halfling.texture_atlas_layout")]
    pub halfling_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "monster.skeleton.texture_atlas_layout")]
    pub skeleton_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "monster.monk.texture_atlas_layout")]
    pub monk_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "monsters.sheets", collection(typed, mapped))]
    pub monsters_sheets: HashMap<String, Handle<Image>>,
    #[asset(key = "resources.sheet")]
    pub resources: Handle<Image>,
    #[asset(key = "resources.texture_atlas_layout")]
    pub resources_layout: Handle<TextureAtlasLayout>,
}

impl GameAssetsHandles {
    pub fn get_character_sheet_handle(&self, name: &str) -> Option<&Handle<Image>> {
        self.characters_sheets
            .iter()
            .find_map(|(sheet, handle)| sheet.contains(name).then(|| handle))
    }

    pub fn get_monster_sheet_handle(&self, name: &str) -> Option<&Handle<Image>> {
        self.monsters_sheets
            .iter()
            .find_map(|(sheet, handle)| sheet.contains(name).then(|| handle))
    }
}

#[derive(AssetCollection, Resource)]
pub struct ConfigHandles {
    #[asset(path = "config.animations.ron")]
    pub animations: Handle<AnimationsConfig>,
}
