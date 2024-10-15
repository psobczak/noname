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
            LoadingStateConfig::new(MyStates::AssetLoading).load_collection::<EntitiesHandle>(), // .load_collection::<CharactersHandle>(),
        )
        .add_plugins(RonAssetPlugin::<Entities>::new(&["config.ron"]));
    }
}

#[derive(AssetCollection, Resource)]
pub struct EntitiesHandle {
    #[asset(path = "characters.config.ron")]
    pub handle: Handle<Entities>,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Debug)]
pub struct Entities {
    pub playable: PlayableCharacters,
    pub monsters: HashMap<String, Monster>,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Debug)]
pub struct Character {
    pub texture: String,
    pub portrait: u32,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Debug)]
pub struct PlayableCharacters {
    pub characters: HashMap<String, Character>,
    pub animations: Animations,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Debug)]
pub struct Monster {
    pub portrait: usize,
    pub texture: String,
    pub animations: Animations,
}

#[derive(
    serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Debug, DerefMut, Deref,
)]
pub struct Animations(pub HashMap<String, Vec<usize>>);
