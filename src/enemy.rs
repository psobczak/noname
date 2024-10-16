use bevy::{prelude::*, window::PrimaryWindow};
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
        app.insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(
                Update,
                (
                    move_towards_player,
                    spawn_halfling,
                    compare_x_pos_with_player,
                )
                    .distributive_run_if(
                        in_state(GameState::Next).and_then(any_with_component::<Player>),
                    ),
            );
    }
}

#[derive(Debug, Component)]
pub struct Enemy;

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

fn compare_x_pos_with_player(
    mut commands: Commands,
    player: Query<&GlobalTransform, With<Player>>,
    enemies: Query<(&GlobalTransform, Entity), With<Enemy>>,
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

fn spawn_halfling(
    window: Query<&Window, With<PrimaryWindow>>,
    player: Query<&GlobalTransform, With<Player>>,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    mut commands: Commands,
    monsters_handles: Res<GameAssetsHandles>,
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
                Enemy,
                Speed(50.0),
                Health(30),
                SpriteBundle {
                    texture: monsters_handles
                        .get_monster_sheet_handle("halfling")
                        .unwrap()
                        .clone(),
                    transform: Transform::from_translation(spawn_point.extend(0.0)),
                    ..Default::default()
                },
                TextureAtlas::from(monsters_handles.halfling_layout.clone()),
            ))
            .observe(on_direction_changed);
    }
}

fn move_towards_player(
    player: Query<&GlobalTransform, With<Player>>,
    mut enemies: Query<(&mut Transform, &Speed), With<Enemy>>,
    time: Res<Time>,
) {
    let player_transform: &GlobalTransform = player.single();
    for (mut enemy_transform, speed) in &mut enemies {
        let direction = enemy_transform.looking_at(player_transform.translation(), Vec3::Y);
        enemy_transform.translation += direction.forward() * time.delta_seconds() * speed.0;
    }
}
