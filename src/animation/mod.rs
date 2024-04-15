use bevy::prelude::*;

pub mod systems;

#[derive(Component, Copy, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub struct SpriteLayout {
    pub columns: usize,
    pub rows: usize,
    pub width: f32,
    pub height: f32
}

#[derive(Component)]
pub struct AnimatedEntity {
    pub texture: Handle<Image>,
    pub animation_indices: AnimationIndices,
    pub sprite_layout: SpriteLayout
}

impl AnimatedEntity {
    pub fn get_texture(&self) -> &Handle<Image> {
        &self.texture
    }

    pub fn get_animation_incides(&self)-> AnimationIndices {
        self.animation_indices
    }
}