mod animation;
mod assets;
mod common;
mod enemy;
mod player;

use animation::AnimationPlugin;
use assets::GameAssetsPlugin;
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use enemy::EnemyPlugin;
use player::PlayerPlugin;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(
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
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading).continue_to_state(MyStates::Next),
        )
        .add_plugins((PlayerPlugin, EnemyPlugin, AnimationPlugin, GameAssetsPlugin))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Slash)),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum MyStates {
    #[default]
    AssetLoading,
    Next,
}
