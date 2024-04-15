use bevy::prelude::*;

#[derive(Component)]
pub struct Mouse {}

#[derive(Debug)]
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
            speed: 70.0,
            state: MovementState::Idle
        }
    }
}

impl MouseMovement {
    pub fn set_state(&mut self, state: MovementState) -> () {
        self.state = state;
    }
}