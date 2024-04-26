use bevy::prelude::*;

pub mod systems;

#[derive(Resource)]
pub struct LevelScroll {
    pub left: f32
}

impl Default for LevelScroll {
    fn default() -> LevelScroll {
        LevelScroll { left: 0.0 }
    }
}

#[derive(Component)]
pub struct LevelElement {}

#[derive(Component)]
pub struct StaticElement {
    pub size: (f32, f32)
}