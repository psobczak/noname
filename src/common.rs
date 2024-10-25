use bevy::prelude::{Component, Deref, DerefMut};

#[derive(Debug, Component, Deref, DerefMut)]
pub struct Speed(pub f32);

#[derive(Debug, Component, Deref, DerefMut)]
pub struct Health(pub u32);
