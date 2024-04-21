use bevy::prelude::*;

pub mod systems;

#[derive(Component)]
pub struct Level {
    pub left: f32
}