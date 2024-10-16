use bevy::prelude::*;

use crate::{common::Speed, GameState};

use super::Player;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DirectionChanged>().add_systems(
            Update,
            move_player.run_if(in_state(GameState::Next).and_then(any_with_component::<Player>)),
        );
    }
}

fn move_player(
    mut player: Query<(&mut Transform, &Speed, &mut MovementDirection, Entity), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let (mut transform, speed, mut old_direction, entity) =
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
        commands.trigger_targets(DirectionChanged(new_direction.clone()), entity);
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
}

#[derive(Event, Debug)]
pub struct DirectionChanged(pub MovementDirection);
