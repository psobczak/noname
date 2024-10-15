use bevy::prelude::*;
use bevy_spritesheet_animation::{library::AnimationLibrary, prelude::SpritesheetAnimation};

use crate::{
    animation::AnimationTimer,
    common::{Health, Speed},
    MyStates,
};

use super::{movement::MovementDirection, Player, PlayerAssets};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyStates::Next), (spawn_player, spawn_other_player));
    }
}

fn spawn_player(
    mut commands: Commands,
    camera: Query<Entity, (With<Camera>, With<Camera2d>)>,
    assets: Res<PlayerAssets>,
) {
    let camera = camera.single();
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
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            MovementDirection::default(),
        ))
        .add_child(camera);
}

#[derive(Component)]
struct OtherPlayer;

fn spawn_other_player(
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    library: Res<AnimationLibrary>,
) {
    let animation_id = library.animation_with_name("player_running_left");
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
