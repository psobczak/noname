use avian2d::prelude::{Collider, CollidingEntities, Collision, CollisionStarted};
use bevy::log::tracing_subscriber::fmt::writer;
use bevy::utils::info;
use bevy::{math::VectorSpace, prelude::*};
use bevy_spatial::kdtree::KDTree2;
use bevy_spatial::{AutomaticUpdate, SpatialAccess, SpatialStructure};

use crate::common::Health;
use crate::{
    enemy::{Dying, Enemy, NearestNeighbour},
    GameState,
};

use super::Player;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyHit>()
            .add_systems(
                OnEnter(GameState::Next),
                spawn_orb.run_if(any_with_component::<Player>),
            )
            .add_systems(
                Update,
                (rotate_orb, deal_damage_to_enemey).run_if(in_state(GameState::Next)),
            )
            .add_systems(
                Update,
                detect_collision_with_enemy.run_if(any_with_component::<Player>),
            );
    }
}

#[derive(Component, Debug)]
enum Weapon {
    Orb { damage: u32, rotation_speed: f32 },
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
            Name::from("Orb"),
            Weapon::Orb {
                damage: 10,
                rotation_speed: 5.0,
            },
            ColorMesh2dBundle {
                transform: Transform::from_xyz(
                    player_translation.x + 70.0,
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

fn rotate_orb(mut weapons: Query<(&Weapon, &mut Transform)>) {
    for (weapon, mut transform) in &mut weapons {
        if let Weapon::Orb { rotation_speed, .. } = weapon {
            // Vec3::ZERO because point is relative to player
            transform.rotate_around(
                Vec3::ZERO,
                Quat::from_rotation_z(rotation_speed.to_radians()),
            );
        }
    }
}

#[derive(Event, Debug)]
struct EnemyHit {
    enemy: Entity,
    damage: u32,
}

fn deal_damage_to_enemey(
    mut commands: Commands,
    mut reader: EventReader<EnemyHit>,
    mut enemies: Query<(Entity, &mut Health), (With<Enemy>, Without<Dying>)>,
) {
    for attack in &mut reader.read() {
        if let Ok((enemy, mut health)) = enemies.get_mut(attack.enemy) {
            health.0 -= attack.damage;

            if health.0 == 0 {
                commands.entity(attack.enemy).insert(Dying);
                commands.entity(enemy).remove::<Collider>();
            }
        }
    }
}

fn detect_collision_with_enemy(
    mut writer: EventWriter<EnemyHit>,
    mut collision_event_reader: EventReader<CollisionStarted>,
    alive_enemies: Query<Entity, (With<Enemy>, With<Collider>, Without<Dying>)>,
    orb: Query<&Weapon>,
) {
    for CollisionStarted(first, second) in collision_event_reader.read() {
        match (orb.get(*first), orb.get(*second)) {
            (Ok(orb), Err(_)) | (Err(_), Ok(orb)) => {
                if let Weapon::Orb { damage, .. } = orb {
                    if let Ok(enemy) = alive_enemies.get(*second) {
                        writer.send(EnemyHit {
                            enemy,
                            damage: *damage,
                        });
                    }
                }
            }
            _ => {}
        }
    }
}
