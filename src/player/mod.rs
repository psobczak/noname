mod movement;
mod spawn;

use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::{ConfigureLoadingState, LoadingStateConfig},
        LoadingStateAppExt,
    },
};
use movement::MovementPlugin;
use spawn::SpawnPlugin;

use bevy::prelude::*;

use crate::MyStates;

pub use movement::{DirectionChanged, MovementDirection};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MovementPlugin, SpawnPlugin))
            .configure_loading_state(
                LoadingStateConfig::new(MyStates::AssetLoading).load_collection::<PlayerAssets>(),
            );
    }
}

#[derive(Debug, Component)]
pub struct Player;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "heroes.png")]
    pub heroes: Handle<Image>,
    #[asset(texture_atlas_layout(
        tile_size_x = 96,
        tile_size_y = 64,
        columns = 45,
        rows = 1,
        padding_y = 5
    ))]
    pub heroes_layut: Handle<TextureAtlasLayout>,
}
