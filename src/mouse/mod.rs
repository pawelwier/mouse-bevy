use bevy::{math::f32, prelude::*};

pub const MOUSE_SIZE: f32 = 64.0;
pub const MOUSE_SCALE: f32 = 2.0;
pub const MOUSE_JUMP: f32 = 9.5;
pub const MOUSE_MARGIN: f32 = 16.0 * MOUSE_SCALE;

use crate::animation::{
    AnimatedEntity, AnimationIndices, SpriteLayout
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
            get_mouse_animated_entity(asset_server, "sprites/mouse_move.png".to_string(), 8),
        MovementState::Jump | MovementState::Fall => {
            get_mouse_animated_entity(asset_server, "sprites/mouse_jump.png".to_string(), 9)
        }
    }
}

#[derive(Component)]
pub struct Mouse {}

#[derive(Debug, PartialEq)]
pub enum MovementState {
    Idle,
    Move,
    Jump,
    Fall
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Still
}

#[derive(Resource)]
pub struct MouseMovement {
    pub speed: f32,
    pub state: MovementState,
    pub jump: f32,
    pub direction: Direction
}

impl Default for MouseMovement {
    fn default() -> MouseMovement {
        MouseMovement {
            speed: 220.0,
            state: MovementState::Idle,
            jump: 0.0,
            direction: Direction::Still
        }
    }
}

impl MouseMovement {
    pub fn set_state(&mut self, state: MovementState) -> () {
        self.state = state;
    }

    pub fn set_jump(&mut self, jump: f32) -> () {
        self.jump = jump;
    }

    pub fn set_direction(&mut self, direction: Direction) -> () {
        self.direction = direction;
    }

    pub fn is_moving(&self) -> bool {
        self.direction == Direction::Left ||self.direction == Direction::Right
    }

    pub fn is_jumping(&self) -> bool {
        self.state == MovementState::Jump || self.state == MovementState::Fall
    }
}