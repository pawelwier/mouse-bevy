use bevy::prelude::*;

use crate::{
    animation::{
        AnimatedEntity, AnimationIndices, SpriteLayout
    }, 
    MOUSE_SIZE
};

fn get_mouse_animated_entity(
    asset_server: Res<AssetServer>,
    path: String,
    frame_count: usize
) -> AnimatedEntity {
    AnimatedEntity {
        texture: asset_server.load(path),
        animation_indices: AnimationIndices { first: 0, last: frame_count - 1 },
        sprite_layout: SpriteLayout {
            columns: frame_count,
            rows: 1,
            width: MOUSE_SIZE,
            height: MOUSE_SIZE
        }
    }   
}

pub fn get_mouse_animation(
    state: &MovementState,
    asset_server: Res<AssetServer>
) -> AnimatedEntity {
    match state {
        MovementState::Idle => 
            get_mouse_animated_entity(asset_server, "sprites/mouse_idle.png".to_string(), 6),
        MovementState::Move => 
            get_mouse_animated_entity(asset_server, "sprites/mouse_move.png".to_string(), 8)
    }
}

#[derive(Component)]
pub struct Mouse {}

#[derive(Debug, PartialEq)]
pub enum MovementState {
    Idle,
    Move,
    // Jump etc. TODO
}
#[derive(Resource)]
pub struct MouseMovement {
    pub speed: f32,
    pub state: MovementState
}

impl Default for MouseMovement {
    fn default() -> MouseMovement {
        MouseMovement {
            speed: 120.0,
            state: MovementState::Idle
        }
    }
}

impl MouseMovement {
    pub fn set_state(&mut self, state: MovementState) -> () {
        self.state = state;
    }
}