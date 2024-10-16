mod animation;
mod assets;
mod common;
mod enemy;
mod player;

use animation::GameAnimationPlugin;
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{assets::GameAssetsPlugin, enemy::EnemyPlugin, player::PlayerPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "NONAME".into(),

                        ..Default::default()
                    }),

                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Next),
        )
        .add_plugins((
            PlayerPlugin,
            GameAnimationPlugin,
            EnemyPlugin,
            GameAssetsPlugin,
        ))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Slash)),
        )
        .add_systems(Startup, setup);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Next,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
