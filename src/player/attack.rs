use avian2d::prelude::{Collider, CollisionStarted};
use bevy::{math::VectorSpace, prelude::*};
use bevy_hanabi::prelude::*;

use crate::common::Health;
use crate::{
    enemy::{Dying, Enemy},
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
                (
                    rotate_orb,
                    deal_damage_to_enemey,
                    insert_flash_duration_timer,
                    change_color_to_red,
                    change_color_to_normal,
                )
                    .run_if(in_state(GameState::Next)),
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
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let (player, transform) = player.single();

    // Define a color gradient from red to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 0., 0., 1.));
    gradient.add_key(1.0, Vec4::splat(0.));

    // Create a new expression module
    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.05),
        dimension: ShapeDimension::Surface,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(6.),
    };

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(10.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Every frame, add a gravity-like acceleration downward
    let accel = module.lit(Vec3::new(0., -3., 0.));
    let update_accel = AccelModifier::new(accel);

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        vec![32768],
        // Spawn at a rate of 5 particles per second
        Spawner::rate(100.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("MyEffect")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .update(update_accel)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(ColorOverLifetimeModifier { gradient });

    // Insert into the asset system
    let effect_handle = effects.add(effect);

    commands.entity(player).with_children(|parent| {
        let player_translation = transform.translation();
        parent
            .spawn((
                Name::from("Orb"),
                Weapon::Orb {
                    damage: 10,
                    rotation_speed: 7.0,
                },
                ColorMesh2dBundle {
                    transform: Transform::from_xyz(
                        player_translation.x + 70.0,
                        player_translation.y,
                        player_translation.z,
                    ),
                    mesh: meshes.add(Circle::new(10.0)).into(),
                    ..Default::default()
                },
                Collider::circle(10.0),
            ))
            .with_children(|parent| {
                parent.spawn(ParticleEffectBundle {
                    effect: ParticleEffect::new(effect_handle.clone()),
                    ..Default::default()
                });
            });
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

#[derive(Component, Deref, DerefMut)]
struct FlashDurationTimer(Timer);

#[derive(Component)]
struct OldColor(Color);

fn insert_flash_duration_timer(mut commands: Commands, mut reader: EventReader<EnemyHit>) {
    for event in reader.read() {
        commands
            .entity(event.enemy)
            .insert(FlashDurationTimer(Timer::from_seconds(
                0.1,
                TimerMode::Once,
            )));
    }
}

fn change_color_to_red(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Sprite), Added<FlashDurationTimer>>,
) {
    for (enemy, mut sprite) in &mut enemies {
        let old_color = OldColor(sprite.color);
        sprite.color = LinearRgba::RED.into();
        commands.entity(enemy).insert(old_color);
    }
}

fn change_color_to_normal(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut FlashDurationTimer, &mut Sprite, &OldColor), (With<Enemy>,)>,
    time: Res<Time>,
) {
    for (enemy, mut timer, mut sprite, old_color) in &mut enemies {
        timer.tick(time.delta());

        if timer.just_finished() {
            sprite.color = old_color.0;
            commands
                .entity(enemy)
                .remove::<FlashDurationTimer>()
                .remove::<OldColor>();
        }
    }
}
