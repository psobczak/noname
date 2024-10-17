use bevy::prelude::*;
use bevy_spritesheet_animation::{library::AnimationLibrary, prelude::SpritesheetAnimation};

use crate::{assets::GameAssetsHandles, GameState};

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Next), spawn_treasure);
    }
}

#[derive(Component, Debug)]
pub enum Resource {
    Treasure,
    Gold,
    Crystals,
}

fn spawn_treasure(
    mut commands: Commands,
    handles: Res<GameAssetsHandles>,
    animations: Res<AnimationLibrary>,
) {
    commands.spawn((
        Resource::Gold,
        SpriteBundle {
            texture: handles.resources.clone(),
            ..Default::default()
        },
        TextureAtlas::from(handles.resources_layout.clone()),
        SpritesheetAnimation::from_id(animations.animation_with_name("jewels_blink").unwrap()),
    ));
}
