use bevy::prelude::*;

use crate::mouse::{
    Mouse, MouseMovement, MovementState
};

pub fn key_pressed (
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    keycode: KeyCode
) -> bool { keyboard_input.pressed(keycode) }

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_query: Query<&mut Transform, With<Mouse>>,
    time: Res<Time>,
    mut mouse_movement: ResMut<MouseMovement>,
) {
    if let Ok(mut transform) = mouse_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let mut x = 0.0;

        if key_pressed(&keyboard_input, KeyCode::ArrowLeft) { x = -1.0; } 
        if key_pressed(&keyboard_input, KeyCode::ArrowRight) { x = 1.0; }

        if x != 0.0 {
            mouse_movement.set_state(MovementState::Move);
            direction += Vec3::new(x, 0.0, 0.0);
            direction = direction.normalize();
        } else {
            mouse_movement.set_state(MovementState::Idle);
        }
        
        transform.translation += direction * 80.0 * time.delta_seconds();        
    }
}
