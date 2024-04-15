use bevy::prelude::*;

pub mod systems;

pub fn key_pressed (
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    keycode: KeyCode
) -> bool { keyboard_input.pressed(keycode) }