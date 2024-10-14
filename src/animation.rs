use bevy::prelude::*;
use bevy_spritesheet_animation::{
    animation::Animation, clip::Clip, library::AnimationLibrary, plugin::SpritesheetAnimationPlugin,
};

use crate::{
    player::{DirectionChanged, MovementDirection, Player},
    MyStates,
};

#[derive(Debug, Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpritesheetAnimationPlugin)
            .add_systems(OnEnter(MyStates::Next), setup)
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

fn setup(mut library: ResMut<AnimationLibrary>) {
    let idle = Clip::from_frames([0]);
    let running_right = Clip::from_frames(21..=28);
    let running_down = Clip::from_frames(5..=12);
    let running_up = Clip::from_frames(37..=44);
    let running_up_right = Clip::from_frames(29..=36);
    let running_down_right = Clip::from_frames(13..=20);

    let idle = Animation::from_clip(library.register_clip(idle));
    let running_right = Animation::from_clip(library.register_clip(running_right));
    let running_down = Animation::from_clip(library.register_clip(running_down));
    let running_up = Animation::from_clip(library.register_clip(running_up));
    let running_up_right = Animation::from_clip(library.register_clip(running_up_right));
    let running_down_right = Animation::from_clip(library.register_clip(running_down_right));

    let idle_id = library.register_animation(idle);
    let running_right_id = library.register_animation(running_right);
    let running_down_id = library.register_animation(running_down);
    let running_up_id = library.register_animation(running_up);
    let running_up_right_id = library.register_animation(running_up_right);
    let running_down_right_id = library.register_animation(running_down_right);

    library.name_animation(idle_id, "walk").unwrap();
    library
        .name_animation(running_right_id, "running_right")
        .unwrap();
    library
        .name_animation(running_down_id, "running_down")
        .unwrap();
    library.name_animation(running_up_id, "running_up").unwrap();
    library
        .name_animation(running_up_right_id, "running_up_right")
        .unwrap();
    library
        .name_animation(running_down_right_id, "running_down_right")
        .unwrap();
}
