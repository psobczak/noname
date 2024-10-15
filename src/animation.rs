use bevy::prelude::*;
use bevy_spritesheet_animation::{
    animation::Animation, clip::Clip, library::AnimationLibrary, plugin::SpritesheetAnimationPlugin,
};

use crate::{
    assets::{Entities, EntitiesHandle},
    player::{DirectionChanged, MovementDirection, Player},
    MyStates,
};

#[derive(Debug, Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpritesheetAnimationPlugin)
            .add_systems(OnExit(MyStates::AssetLoading), setup)
            .add_systems(
                Update,
                (animate_movement, change_sprite_index, animate_movement)
                    .run_if(in_state(MyStates::Next)),
            );
    }
}

fn change_sprite_index(
    mut reader: EventReader<DirectionChanged>,
    mut sprite: Query<(&mut TextureAtlas, &mut Sprite), With<Player>>,
) {
    let (mut layout, mut sprite) = sprite.single_mut();
    for event in reader.read() {
        let indices = event.0.get_animation_indices();

        layout.index = *indices.first().unwrap();

        match event.0 {
            MovementDirection::DownLeft | MovementDirection::UpLeft | MovementDirection::Left => {
                sprite.flip_x = true
            }
            MovementDirection::DownRight
            | MovementDirection::RightUp
            | MovementDirection::Right => sprite.flip_x = false,
            _ => {}
        }
    }
}

fn animate_movement(
    mut sprites_to_animate: Query<(&mut TextureAtlas, &mut AnimationTimer, &MovementDirection)>,
    time: Res<Time>,
) {
    for (mut sprite, mut timer, direction) in &mut sprites_to_animate {
        timer.0.tick(time.delta());
        let indices = direction.get_animation_indices();
        if timer.finished() {
            if sprite.index >= *indices.last().unwrap() {
                sprite.index = *indices.first().unwrap()
            } else {
                sprite.index = sprite.index + 1;
            }
        }
    }
}

fn setup(
    mut library: ResMut<AnimationLibrary>,
    mut entities: ResMut<Assets<Entities>>,
    handle: Res<EntitiesHandle>,
) {
    if let Some(entities) = entities.get_mut(handle.handle.id()) {
        let animations = &mut entities.playable.animations;
        let _ = &entities.monsters.iter().for_each(|(_, monster)| {
            monster
                .animations
                .0
                .iter()
                .for_each(|(animation_name, frames)| {
                    animations
                        .0
                        .entry(animation_name.to_string())
                        .insert(frames.to_vec());
                });
        });

        animations.iter().for_each(|(animation_name, frames)| {
            let clip = Clip::from_frames(frames.clone());
            let clip_id = library.register_clip(clip);
            let animation = Animation::from_clip(clip_id);
            let animation_id = library.register_animation(animation);
            library
                .name_animation(animation_id, animation_name)
                .unwrap();
        });

        info!("{:#?}", animations);
    }
}
