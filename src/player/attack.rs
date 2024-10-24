use std::f32::consts::TAU;

use avian2d::prelude::Collider;
use bevy::{prelude::*, sprite::Mesh2d};

use crate::GameState;

use super::Player;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Next),
            spawn_orb.run_if(any_with_component::<Player>),
        )
        .add_systems(Update, rotate_orb.run_if(in_state(GameState::Next)));
    }
}

#[derive(Component, Debug)]
enum Weapon {
    Orb { radius: f32, count: u32 },
    Sword,
    Arrow,
}

fn spawn_orb(
    mut commands: Commands,
    player: Query<(Entity, &GlobalTransform), With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let (player, transform) = player.single();

    commands.entity(player).with_children(|parent| {
        let player_translation = transform.translation();
        parent.spawn((
            Weapon::Orb {
                radius: 30.0,
                count: 1,
            },
            ColorMesh2dBundle {
                transform: Transform::from_xyz(
                    player_translation.x + 50.0,
                    player_translation.y,
                    player_translation.z,
                ),
                mesh: meshes.add(Circle::new(5.0)).into(),
                ..Default::default()
            },
            Collider::circle(5.0),
        ));
    });
}

fn rotate_orb(
    mut weapons: Query<(&Weapon, &mut Transform)>,
    player: Query<&GlobalTransform, With<Player>>,
    time: Res<Time>,
) {
    let player = player.single();
    for (weapon, mut transform) in &mut weapons {
        if let Weapon::Orb { radius, count } = weapon {
            transform.rotate_y(1.0 * TAU * time.delta_seconds());
        }
    }
}
