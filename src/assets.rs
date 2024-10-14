use std::path::PathBuf;

use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::{ConfigureLoadingState, LoadingStateConfig},
        LoadingStateAppExt,
    },
};
use bevy_common_assets::ron::RonAssetPlugin;

use crate::MyStates;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(MyStates::AssetLoading).load_collection::<EntitiesHandle>(),
        )
        .add_plugins(RonAssetPlugin::<Entities>::new(&["config.ron"]));
    }
}

#[derive(AssetCollection, Resource)]
pub struct EntitiesHandle {
    #[asset(path = "entities.config.ron")]
    pub entities: Handle<Entities>,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Debug)]
pub struct Entities {
    pub entities: HashMap<String, Entity>,
}

impl Entities {
    pub fn get_entity(&self, name: &str) -> Option<&Entity> {
        self.entities.get(name)
    }
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Debug)]
pub struct Entity {
    pub sprite: PathBuf,
    pub sprite_sheet: Option<SpriteSheet>,
    pub animations: HashMap<String, Vec<usize>>,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Debug, Clone)]
pub struct SpriteSheet {
    pub tile_size_x: u32,
    pub tile_size_y: u32,
    pub rows: u32,
    pub columns: u32,
}
