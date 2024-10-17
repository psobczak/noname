mod movement;
mod spawn;

use movement::MovementPlugin;
use spawn::SpawnPlugin;

use bevy::prelude::*;

pub use movement::{DirectionChanged, MovementDirection};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MovementPlugin, SpawnPlugin));
    }
}

#[derive(Debug, Component)]
pub struct Player;
