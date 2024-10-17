use avian2d::prelude::{Collider, CollidingEntities};
use bevy::{prelude::*, reflect::Enum};
use bevy_spritesheet_animation::{library::AnimationLibrary, prelude::SpritesheetAnimation};

use crate::{assets::GameAssetsHandles, player::Player, GameState};

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Resource>()
            .add_event::<ResourceCollected>()
            .init_resource::<Resources>()
            .add_systems(OnEnter(GameState::Next), spawn_treasure)
            .add_systems(
                Update,
                (
                    resource_pickup,
                    print_resources.run_if(resource_exists_and_changed::<Resources>),
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
                ..Default::default()
            },
            texture_atlas: TextureAtlas::from(handles.resources_layout.clone()),
            sprite_sheet_animation: SpritesheetAnimation::from_id(animation_id),
            collider: Collider::rectangle(35.0, 35.0),
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

fn print_resources(resources: Res<Resources>) {
    info!("{:?}", resources);
}

fn spawn_treasure(
    mut commands: Commands,
    handles: Res<GameAssetsHandles>,
    animations: Res<AnimationLibrary>,
) {
    let Some(bundle) = ResourceBundle::new(Resource::Gold, &handles, &animations) else {
        return error!("Failed to create resource bundle");
    };

    commands.spawn(bundle);
}
