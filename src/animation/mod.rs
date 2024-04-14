use bevy::prelude::*;

pub mod systems;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);