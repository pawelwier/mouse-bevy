use bevy::prelude::*;

pub mod systems;

pub const GRAVITY: f32 = 98.0;

pub fn key_pressed (
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    keycode: KeyCode
) -> bool { keyboard_input.pressed(keycode) }

pub fn key_just_pressed (
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    keycode: KeyCode
) -> bool { keyboard_input.just_pressed(keycode) }
