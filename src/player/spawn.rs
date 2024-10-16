use bevy::prelude::*;
use bevy_spritesheet_animation::{library::AnimationLibrary, prelude::SpritesheetAnimation};

use crate::{
    common::{Health, Speed},
    GameState,
};

use super::{movement::MovementDirection, DirectionChanged, Player, PlayerAssets};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Next), (spawn_player));
    }
}

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

fn spawn_player(
    mut commands: Commands,
    camera: Query<Entity, (With<Camera>, With<Camera2d>)>,
    animations: Res<AnimationLibrary>,
    assets: Res<PlayerAssets>,
) {
    let camera = camera.single();
    if let Some(idle_id) = animations.animation_with_name("player_idle") {
        commands
            .spawn(PlayerBundle {
                name: Name::from("Player"),
                player: Player,
                speed: Speed(100.0),
                health: Health(100),
                sprite_bundle: SpriteBundle {
                    texture: assets.heroes.clone(),
                    ..Default::default()
                },
                direction: MovementDirection::default(),
                texture_atlas: TextureAtlas::from(assets.heroes_layut.clone()),
                sprite_sheet_animation: SpritesheetAnimation::from_id(idle_id),
            })
            .observe(on_player_direction_changed)
            .add_child(camera);
    }
}

fn on_player_direction_changed(
    trigger: Trigger<DirectionChanged>,
    mut player: Query<&mut Sprite, With<Player>>,
) {
    let mut sprite = player.get_mut(trigger.entity()).unwrap();
    match trigger.event().0 {
        MovementDirection::DownLeft | MovementDirection::UpLeft | MovementDirection::Left => {
            sprite.flip_x = true
        }
        MovementDirection::DownRight | MovementDirection::RightUp | MovementDirection::Right => {
            sprite.flip_x = false
        }
        _ => {}
    }
}
