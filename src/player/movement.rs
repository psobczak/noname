use bevy::prelude::*;

use crate::{common::Speed, MyStates};

use super::Player;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DirectionChanged>().add_systems(
            Update,
            move_player.run_if(in_state(MyStates::Next).and_then(any_with_component::<Player>)),
        );
    }
}

fn move_player(
    mut player: Query<(&mut Transform, &Speed, &mut MovementDirection), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut writer: EventWriter<DirectionChanged>,
) {
    let (mut transform, speed, mut old_direction) =
        player.get_single_mut().expect("Player should exist");

    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::KeyA) {
        direction.x = -1.0;
    }

    if input.pressed(KeyCode::KeyD) {
        direction.x = 1.0;
    }

    if input.pressed(KeyCode::KeyS) {
        direction.y = -1.0;
    }

    if input.pressed(KeyCode::KeyW) {
        direction.y = 1.0;
    }

    let new_direction = MovementDirection::from_vec2(direction);
    if *old_direction != new_direction {
        writer.send(DirectionChanged(new_direction.clone()));
        *old_direction = new_direction;
    }

    transform.translation +=
        direction.extend(0.0).normalize_or_zero() * speed.0 * time.delta_seconds();
}

#[derive(Component, Default, Debug, PartialEq, Clone)]
pub enum MovementDirection {
    Up,
    UpLeft,
    Left,
    DownLeft,
    #[default]
    Idle,
    Down,
    DownRight,
    Right,
    RightUp,
}

impl MovementDirection {
    pub fn from_vec2(direction: Vec2) -> Self {
        match direction.as_ref() {
            [0.0, 1.0] => Self::Up,
            [-1.0, 1.0] => Self::UpLeft,
            [-1.0, 0.0] => Self::Left,
            [-1.0, -1.0] => Self::DownLeft,
            [0.0, -1.0] => Self::Down,
            [1.0, -1.0] => Self::DownRight,
            [1.0, 0.0] => Self::Right,
            [1.0, 1.0] => Self::RightUp,
            _ => Self::Idle,
        }
    }

    pub fn get_animation_indices(&self) -> Vec<usize> {
        match self {
            MovementDirection::Down => 5..=12,
            MovementDirection::Up => 37..=44,
            MovementDirection::UpLeft | MovementDirection::RightUp => 29..=36,
            MovementDirection::DownLeft | MovementDirection::DownRight => 13..=20,
            MovementDirection::Right | MovementDirection::Left => 21..=28,
            MovementDirection::Idle => 0..=0,
        }
        .into_iter()
        .collect()
    }
}

#[derive(Event, Debug)]
pub struct DirectionChanged(pub MovementDirection);
