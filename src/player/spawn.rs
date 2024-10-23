use avian2d::prelude::Collider;
use bevy::prelude::*;
use bevy_spritesheet_animation::{library::AnimationLibrary, prelude::SpritesheetAnimation};

use crate::{
    assets::GameAssetsHandles,
    common::{Health, Speed},
    GameState,
};

use super::{movement::MovementDirection, DirectionChanged, Player};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), spawn_player);
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
    collider: Collider,
}

fn spawn_player(
    mut commands: Commands,
    camera: Query<Entity, (With<Camera>, With<Camera2d>)>,
    animations: Res<AnimationLibrary>,
    handles: Res<GameAssetsHandles>,
) {
    let camera = camera.single();
    let Some(sheet_handle) = handles.get_character_sheet_handle("cleric") else {
        panic!("player sheet should be present at this point");
    };

    if let Some(idle_id) = animations.animation_with_name("player_idle") {
        commands
            .spawn(PlayerBundle {
                name: Name::from("Player"),
                player: Player,
                speed: Speed(100.0),
                health: Health(100),
                sprite_bundle: SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 10.0),
                    texture: sheet_handle.clone(),
                    ..Default::default()
                },
                direction: MovementDirection::default(),
                texture_atlas: TextureAtlas::from(handles.characters_layouts.clone()),
                sprite_sheet_animation: SpritesheetAnimation::from_id(idle_id),
                collider: Collider::rectangle(30.0, 35.0),
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
