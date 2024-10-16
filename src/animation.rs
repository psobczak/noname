use bevy::prelude::*;
use bevy_spritesheet_animation::{
    animation::Animation, clip::Clip, library::AnimationLibrary,
    plugin::SpritesheetAnimationPlugin, prelude::SpritesheetAnimation,
};

use crate::{
    assets::{AnimationsConfig, ConfigHandles},
    player::{DirectionChanged, MovementDirection, Player},
    GameState,
};

pub struct GameAnimationPlugin;

impl Plugin for GameAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpritesheetAnimationPlugin)
            .add_systems(OnExit(GameState::AssetLoading), load_animations)
            .observe(on_player_direction_changed);
    }
}

fn on_player_direction_changed(
    trigger: Trigger<DirectionChanged>,
    mut player: Query<&mut SpritesheetAnimation, With<Player>>,
    library: Res<AnimationLibrary>,
) {
    let mut animation = player.single_mut();
    match trigger.event().0 {
        MovementDirection::Up => {
            if let Some(anim) = library.animation_with_name("player_running_up") {
                animation.switch(anim);
            }
        }
        MovementDirection::UpLeft => {
            if let Some(anim) = library.animation_with_name("player_running_up_left") {
                animation.switch(anim);
            }
        }
        MovementDirection::Left => {
            if let Some(anim) = library.animation_with_name("player_running_left") {
                animation.switch(anim);
            }
        }
        MovementDirection::DownLeft => {
            if let Some(anim) = library.animation_with_name("player_running_down_left") {
                animation.switch(anim);
            }
        }
        MovementDirection::Idle => {
            if let Some(anim) = library.animation_with_name("player_idle") {
                animation.switch(anim);
            }
        }
        MovementDirection::Down => {
            if let Some(anim) = library.animation_with_name("player_running_down") {
                animation.switch(anim);
            }
        }
        MovementDirection::DownRight => {
            if let Some(anim) = library.animation_with_name("player_running_down_right") {
                animation.switch(anim);
            }
        }
        MovementDirection::Right => {
            if let Some(anim) = library.animation_with_name("player_running_right") {
                animation.switch(anim);
            }
        }
        MovementDirection::RightUp => {
            if let Some(anim) = library.animation_with_name("player_running_up_right") {
                animation.switch(anim);
            }
        }
    }
}

fn load_animations(
    mut library: ResMut<AnimationLibrary>,
    mut animatins_config: ResMut<Assets<AnimationsConfig>>,
    handle: Res<ConfigHandles>,
) {
    if let Some(animatins_config) = animatins_config.get_mut(handle.animations.id()) {
        animatins_config.0.iter().for_each(|(_, animations)| {
            animations.iter().for_each(|(animation_name, frames)| {
                let clip = Clip::from_frames(frames.clone());
                let clip_id = library.register_clip(clip);
                let animation = Animation::from_clip(clip_id);
                let animation_id = library.register_animation(animation);
                library
                    .name_animation(animation_id, animation_name)
                    .unwrap();
            })
        });
    }
}
