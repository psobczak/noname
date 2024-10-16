mod movement;
mod spawn;

use bevy_spritesheet_animation::prelude::SpritesheetAnimation;
use movement::MovementPlugin;
use spawn::SpawnPlugin;

use bevy::prelude::*;

pub use movement::{DirectionChanged, MovementDirection};

use crate::common::{Health, Speed};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MovementPlugin, SpawnPlugin));
    }
}

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Bundle)]
pub struct PlayerBundle {
    name: Name,
    player: Player,
    speed: Speed,
    sprite_bundle: SpriteBundle,
    health: Health,
    direction: MovementDirection,
    texture_atlas: TextureAtlas,
    sprite_sheet_animation: SpritesheetAnimation,
}
