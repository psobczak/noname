use bevy::prelude::*;

use crate::{
    animation::AnimationTimer,
    common::{Health, Speed},
    MyStates,
};

use super::{movement::MovementDirection, Player, PlayerAssets};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyStates::Next), spawn_player);
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
