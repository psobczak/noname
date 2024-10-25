use std::time::Duration;

use avian2d::collision::Collider;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_spatial::{kdtree::KDTree2, AutomaticUpdate, SpatialAccess, SpatialStructure};
use bevy_spritesheet_animation::{
    events::AnimationEvent, library::AnimationLibrary, prelude::SpritesheetAnimation,
};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{
    assets::GameAssetsHandles,
    common::{Health, Speed},
    player::Player,
    GameState,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::new(
            Duration::from_millis(50),
            TimerMode::Repeating,
        )))
        .add_plugins(
            AutomaticUpdate::<NearestNeighbour>::new().with_spatial_ds(SpatialStructure::KDTree2),
        )
        .add_systems(
            Update,
            (
                move_towards_player,
                spawn_enemy,
                enemy_direction_change,
                on_dying,
                on_death_animation_end,
                add_colliders_to_close_enemies,
                kill_all_on_screen,
            )
                .distributive_run_if(
                    in_state(GameState::Next).and_then(any_with_component::<Player>),
                ),
        );
    }
}

#[derive(Bundle, Debug)]
pub struct EnemyBundle {
    name: Name,
    enemy: Enemy,
    speed: Speed,
    health: Health,
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    sprite_sheet_animation: SpritesheetAnimation,
}

impl EnemyBundle {
    fn new(
        name: &str,
        speed: f32,
        health: u32,
        spawn_point: Vec3,
        monsters_handles: &GameAssetsHandles,
        animations: &AnimationLibrary,
    ) -> Option<Self> {
        let texture_atlas_layout: &Handle<TextureAtlasLayout> =
            monsters_handles.get_field(&format!("{name}_layout"))?;
        Some(Self {
            name: Name::from(name),
            speed: Speed(speed),
            health: Health(health),
            enemy: Enemy,
            sprite_bundle: SpriteBundle {
                texture: monsters_handles.get_monster_sheet_handle(name)?.clone(),
                transform: Transform::from_translation(spawn_point),
                ..Default::default()
            },
            texture_atlas: TextureAtlas::from(texture_atlas_layout.clone()),
            sprite_sheet_animation: SpritesheetAnimation::from_id(
                animations.animation_with_name(format!("{name}_walk"))?,
            ),
        })
    }
}

#[allow(clippy::type_complexity)]
fn add_colliders_to_close_enemies(
    mut commands: Commands,
    close_enemies: Res<KDTree2<NearestNeighbour>>,
    enemies_without_collider: Query<Entity, (With<Enemy>, Without<Collider>, Without<Dying>)>,
    player: Query<&GlobalTransform, With<Player>>,
) {
    let player = player.single();
    let close_enemies = close_enemies.within_distance(player.translation().truncate(), 200.0);
    for (_, enemy) in &close_enemies {
        if let Some(enemy) = enemy {
            if let Ok(enemy) = enemies_without_collider.get(*enemy) {
                commands
                    .entity(enemy)
                    .insert(Collider::rectangle(15.0, 45.0));
            }
        }
    }
}

#[derive(Debug, Component)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut)]
struct DotTimer(Timer);

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Debug)]
enum SpawnDirection {
    North,
    South,
    West,
    East,
}

impl SpawnDirection {
    fn calculate_x(&self, window: &Window, player: &Vec3) -> Option<f32> {
        match self {
            SpawnDirection::North => None,
            SpawnDirection::South => None,
            SpawnDirection::West => Some((player.x - window.width() / 2.0) - 30.0),
            SpawnDirection::East => Some((player.x + window.width() / 2.0) + 30.0),
        }
    }

    fn calculate_y(&self, window: &Window, player: &Vec3) -> Option<f32> {
        match self {
            SpawnDirection::South => Some((player.y - window.height() / 2.0) - 30.0),
            SpawnDirection::North => Some((player.y + window.height() / 2.0) + 30.0),
            SpawnDirection::West => None,
            SpawnDirection::East => None,
        }
    }
}

impl Distribution<SpawnDirection> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> SpawnDirection {
        match rng.gen_range(0..=3) {
            0 => SpawnDirection::East,
            1 => SpawnDirection::North,
            2 => SpawnDirection::South,
            _ => SpawnDirection::West,
        }
    }
}

#[derive(Event)]
enum SpriteDirection {
    Left,
    Right,
}

#[allow(clippy::type_complexity)]
fn enemy_direction_change(
    mut commands: Commands,
    player: Query<&GlobalTransform, With<Player>>,
    enemies: Query<(&GlobalTransform, Entity), (With<Enemy>, Without<Dying>)>,
) {
    let player = player.single();
    for (enemy, entity) in &enemies {
        if enemy.translation().x < player.translation().x {
            commands.trigger_targets(SpriteDirection::Right, entity)
        } else {
            commands.trigger_targets(SpriteDirection::Left, entity)
        }
    }
}

fn on_direction_changed(
    trigger: Trigger<SpriteDirection>,
    mut sprites: Query<&mut Sprite, With<Enemy>>,
) {
    let mut sprite = sprites.get_mut(trigger.entity()).unwrap();
    match trigger.event() {
        SpriteDirection::Left => sprite.flip_x = true,
        SpriteDirection::Right => sprite.flip_x = false,
    }
}

fn spawn_enemy(
    window: Query<&Window, With<PrimaryWindow>>,
    player: Query<&GlobalTransform, With<Player>>,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    mut commands: Commands,
    monsters_handles: Res<GameAssetsHandles>,
    animations: Res<AnimationLibrary>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let player = player.single().translation();
        let window = window.single();

        let mut rng = rand::thread_rng();

        let spawn_direction: SpawnDirection = rand::random();
        let x = spawn_direction.calculate_x(window, &player);
        let y = spawn_direction.calculate_y(window, &player);

        let spawn_point = match (x, y) {
            (Some(x), _) => Vec2::new(
                x,
                rng.gen_range((player.y - window.height() / 2.0)..player.y + window.height() / 2.0),
            ),
            (_, Some(y)) => Vec2::new(
                rng.gen_range((player.x - window.width() / 2.0)..player.x + window.width() / 2.0),
                y,
            ),
            _ => unreachable!("This should never happen"),
        };

        commands
            .spawn((
                EnemyBundle::new(
                    "monk",
                    20.0,
                    40,
                    spawn_point.extend(0.0),
                    &monsters_handles,
                    &animations,
                )
                .unwrap(),
                DotTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
                NearestNeighbour,
            ))
            .observe(on_direction_changed);
    }
}

#[allow(clippy::type_complexity)]
fn move_towards_player(
    player: Query<&GlobalTransform, With<Player>>,
    mut enemies: Query<(&mut Transform, &Speed), (With<Enemy>, Without<Dying>)>,
    time: Res<Time>,
) {
    let player_transform: &GlobalTransform = player.single();
    for (mut enemy_transform, speed) in &mut enemies {
        if player_transform
            .translation()
            .distance(enemy_transform.translation)
            > 10.0
        {
            let direction = enemy_transform.looking_at(player_transform.translation(), Vec3::Y);
            enemy_transform.translation += direction.forward() * time.delta_seconds() * speed.0;
        }
    }
}

#[derive(Component)]
pub struct Dying;

#[derive(Component, Default)]
pub struct NearestNeighbour;

#[allow(clippy::type_complexity)]
fn on_dying(
    mut query: Query<(&mut SpritesheetAnimation, &Name), (Added<Dying>, With<Enemy>)>,
    animations: Res<AnimationLibrary>,
) {
    for (mut sprite_animation, name) in &mut query {
        let death_animation = animations
            .animation_with_name(format!("{}_idle", name))
            .unwrap();

        sprite_animation.switch(death_animation);
    }
}

fn kill_all_on_screen(
    mut commands: Commands,
    enemies: Query<Entity, (With<Enemy>, Without<Dying>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let enemies = enemies
            .into_iter()
            .map(|entity| (entity, Dying))
            .collect::<Vec<_>>();
        commands.insert_or_spawn_batch(enemies);
    }
}

fn on_death_animation_end(
    mut commands: Commands,
    mut events: EventReader<AnimationEvent>,
    dying_enemies: Query<Entity, (With<Enemy>, With<Dying>)>,
) {
    for animation_event in events.read() {
        if let AnimationEvent::AnimationRepetitionEnd {
            animation_repetition,
            entity,
            ..
        } = animation_event
        {
            if animation_repetition == &1 && dying_enemies.get(*entity).is_ok() {
                commands.entity(*entity).despawn_recursive();
            }
        }
    }
}
