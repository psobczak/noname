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
    Gold,
    Crystals,
    Mercury,
    Sulfur,
    Ore,
    Wood,
    Gems,
}

#[derive(Bundle, Debug)]
pub struct ResourceBundle {
    resource: Resource,
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    sprite_sheet_animation: SpritesheetAnimation,
}

impl ResourceBundle {
    fn new(
        resource: Resource,
        handles: &GameAssetsHandles,
        animations: &AnimationLibrary,
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
            resource,
            sprite_bundle: SpriteBundle {
                texture: handles.resources.clone(),
                ..Default::default()
            },
            texture_atlas: TextureAtlas::from(handles.resources_layout.clone()),
            sprite_sheet_animation: SpritesheetAnimation::from_id(animation_id),
        })
    }
}

fn spawn_treasure(
    mut commands: Commands,
    handles: Res<GameAssetsHandles>,
    animations: Res<AnimationLibrary>,
) {
    let Some(bundle) = ResourceBundle::new(Resource::Crystals, &handles, &animations) else {
        return error!("Failed to create resource bundle");
    };

    commands.spawn(bundle);
}
