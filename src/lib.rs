mod animation;
mod assets;
mod common;
mod enemy;
mod player;
mod resources;

use animation::GameAnimationPlugin;
use avian2d::PhysicsPlugins;
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, input::common_conditions::input_toggle_active,
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use bevy_tweening::TweeningPlugin;
use resources::ResourcePlugin;

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
        .add_plugins((
            PlayerPlugin,
            GameAnimationPlugin,
            EnemyPlugin,
            GameAssetsPlugin,
            ResourcePlugin,
        ))
        .add_plugins((
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Slash)),
            FrameTimeDiagnosticsPlugin,
            PhysicsPlugins::default(),
            // PhysicsDebugPlugin::default(),
            TweeningPlugin,
            ScreenDiagnosticsPlugin::default(),
            ScreenFrameDiagnosticsPlugin,
        ))
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
    commands.spawn((Name::from("Camera"), Camera2dBundle::default()));
}
