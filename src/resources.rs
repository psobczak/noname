use std::time::Duration;

use avian2d::prelude::{Collider, CollidingEntities};
use bevy::{prelude::*, reflect::Enum};
use bevy_spritesheet_animation::{
    events::AnimationEvent, library::AnimationLibrary, prelude::SpritesheetAnimation,
};
use bevy_tweening::{
    lens::TransformPositionLens, Animator, EaseFunction, RepeatCount, RepeatStrategy, Tween,
    TweenCompleted,
};
use rand::{distributions::Standard, prelude::Distribution};

use crate::{
    assets::GameAssetsHandles,
    enemy::{Dying, Enemy},
    player::Player,
    GameState,
};

const PICKUP_RANGE: f32 = 70.0;
pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Resource>()
            .add_event::<ResourceCollected>()
            .init_resource::<Resources>()
            .add_systems(
                Update,
                (
                    resource_pickup,
                    on_enemy_killed,
                    (
                        mark_resource_as_close,
                        mark_resource_as_following,
                        update_resource_position,
                    )
                        .chain(),
                )
                    .run_if(in_state(GameState::Next)),
            )
            .observe(on_resource_collected);
    }
}

#[derive(Resource, Debug, Default, Reflect)]
pub struct Resources {
    gold: u32,
    crystals: u32,
    mercury: u32,
    sulfur: u32,
    ore: u32,
    wood: u32,
    gems: u32,
}

#[derive(Component, Debug, Reflect, Clone)]
pub enum Resource {
    Gold,
    Crystals,
    Mercury,
    Sulfur,
    Ore,
    Wood,
    Gems,
}

impl Distribution<Resource> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Resource {
        match rng.gen_range(0.0..=1.0) {
            0.0..0.2 => Resource::Gold,
            0.2..0.4 => Resource::Wood,
            0.4..0.6 => Resource::Sulfur,
            0.6..0.8 => Resource::Ore,
            0.8..0.9 => Resource::Mercury,
            0.9..0.95 => Resource::Crystals,
            _ => Resource::Gems,
        }
    }
}

#[derive(Component)]
struct CloseToPlayer;

#[derive(Component)]
struct FollowingPlayer;

fn mark_resource_as_close(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    resources: Query<
        (Entity, &Transform),
        (Without<Player>, With<Resource>, Without<CloseToPlayer>),
    >,
) {
    let player_transform = player.single();

    for (resource, resource_transofrm) in &resources {
        if resource_transofrm
            .translation
            .distance(player_transform.translation)
            < PICKUP_RANGE
        {
            let tween_direction =
                player_transform.looking_at(resource_transofrm.translation, Vec3::Y);

            let tween = Tween::new(
                EaseFunction::BackIn,
                Duration::from_millis(500),
                TransformPositionLens {
                    start: resource_transofrm.translation,
                    end: tween_direction.translation,
                },
            )
            .with_repeat_count(RepeatCount::Finite(1))
            .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
            .with_completed_event(10);

            commands
                .entity(resource)
                .insert(Animator::new(tween))
                .insert(CloseToPlayer);
        }
    }
}

fn mark_resource_as_following(
    mut commands: Commands,
    mut completed_tweens: EventReader<TweenCompleted>,
    resources: Query<
        Entity,
        (
            With<Resource>,
            With<CloseToPlayer>,
            Without<FollowingPlayer>,
        ),
    >,
) {
    for tween in &mut completed_tweens.read() {
        info!("Tween completed: {:?}", tween.entity);
        if let Ok(tween) = resources.get(tween.entity) {
            commands.entity(tween).insert(FollowingPlayer);
        }
    }
}

fn update_resource_position(
    player: Query<&GlobalTransform, With<Player>>,
    mut resources: Query<&mut Transform, (With<Resource>, With<FollowingPlayer>)>,
    time: Res<Time>,
) {
    let player_transform = player.single();
    for mut transform in &mut resources {
        let direction = transform.looking_at(player_transform.translation(), Vec3::Y);
        transform.translation += direction.forward() * time.delta_seconds() * 250.0;
    }
}

#[derive(Bundle, Debug)]
pub struct ResourceBundle {
    name: Name,
    resource: Resource,
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    sprite_sheet_animation: SpritesheetAnimation,
    collider: Collider,
}

impl ResourceBundle {
    fn new(
        resource: Resource,
        handles: &GameAssetsHandles,
        animations: &AnimationLibrary,
        translation: Vec3,
    ) -> Option<Self> {
        let animation_id = match resource {
            Resource::Gold => animations.animation_with_name("gold_blink")?,
            Resource::Crystals => animations.animation_with_name("crystals_blink")?,
            Resource::Gems => animations.animation_with_name("gems_blink")?,
            Resource::Mercury => animations.animation_with_name("mercury")?,
            Resource::Sulfur => animations.animation_with_name("sulfur")?,
            Resource::Ore => animations.animation_with_name("ore")?,
            Resource::Wood => animations.animation_with_name("wood")?,
        };

        Some(Self {
            name: Name::from(resource.variant_name()),
            resource,
            sprite_bundle: SpriteBundle {
                texture: handles.resources.clone(),
                transform: Transform::from_translation(translation),
                ..Default::default()
            },
            texture_atlas: TextureAtlas::from(handles.resources_layout.clone()),
            sprite_sheet_animation: SpritesheetAnimation::from_id(animation_id),
            collider: Collider::rectangle(15.0, 15.0),
        })
    }
}

#[derive(Event)]
pub struct ResourceCollected {
    resource: Resource,
    amount: u32,
}

fn resource_pickup(
    mut commands: Commands,
    collisions: Query<&CollidingEntities, With<Player>>,
    resources: Query<(Entity, &Resource), Without<Player>>,
) {
    for CollidingEntities(collisions) in collisions.iter() {
        for (entity, resource) in &resources {
            if collisions.contains(&entity) {
                commands.trigger_targets(
                    ResourceCollected {
                        resource: resource.clone(),
                        amount: 1,
                    },
                    entity,
                );
            }
        }
    }
}

fn on_resource_collected(
    trigger: Trigger<ResourceCollected>,
    mut resources: ResMut<Resources>,
    mut commands: Commands,
) {
    let event = trigger.event();
    match event.resource {
        Resource::Gold => resources.gold += event.amount,
        Resource::Crystals => resources.crystals += event.amount,
        Resource::Mercury => resources.mercury += event.amount,
        Resource::Sulfur => resources.sulfur += event.amount,
        Resource::Ore => resources.ore += event.amount,
        Resource::Wood => resources.wood += event.amount,
        Resource::Gems => resources.gems += event.amount,
    }

    commands.entity(trigger.entity()).despawn_recursive();
}

fn on_enemy_killed(
    mut commands: Commands,
    mut animation_events: EventReader<AnimationEvent>,
    handles: Res<GameAssetsHandles>,
    animations: Res<AnimationLibrary>,
    dying_enemies: Query<(Entity, &GlobalTransform), (With<Enemy>, With<Dying>)>,
) {
    for animation_event in animation_events.read() {
        if let AnimationEvent::AnimationRepetitionEnd {
            animation_repetition,
            entity,
            ..
        } = animation_event
        {
            let Ok(killed_enemy) = dying_enemies.get(*entity) else {
                return;
            };

            if animation_repetition == &1 {
                let resource: Resource = rand::random();

                let Some(bundle) = ResourceBundle::new(
                    resource,
                    &handles,
                    &animations,
                    killed_enemy.1.translation(),
                ) else {
                    return error!("Failed to create resource bundle");
                };

                commands.spawn((bundle,));
            }
        }
    }
}
