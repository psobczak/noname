use bevy::prelude::*;
use bevy_spritesheet_animation::{library::AnimationLibrary, prelude::SpritesheetAnimation};

use crate::{
    common::{Health, Speed},
    MyStates,
};

use super::{movement::MovementDirection, DirectionChanged, Player, PlayerAssets};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyStates::Next), (spawn_player, spawn_other_player));
    }
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
            .spawn((
                Player,
                Speed(100.0),
                Health(100),
                SpriteBundle {
                    texture: assets.heroes.clone(),
                    ..Default::default()
                },
                TextureAtlas::from(assets.heroes_layut.clone()),
                MovementDirection::default(),
                SpritesheetAnimation::from_id(idle_id),
            ))
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

#[derive(Component)]
pub struct OtherPlayer;

fn spawn_other_player(
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    library: Res<AnimationLibrary>,
) {
    let animation_id = library.animation_with_name("player_idle");
    if let Some(id) = animation_id {
        commands.spawn((
            OtherPlayer,
            SpriteBundle {
                texture: assets.heroes.clone(),
                transform: Transform::from_xyz(0.0, 100.0, 0.0),
                ..Default::default()
            },
            TextureAtlas {
                layout: assets.heroes_layut.clone(),
                ..Default::default()
            },
            SpritesheetAnimation::from_id(id),
        ));
    }
}
