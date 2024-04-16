use bevy::prelude::*;

use crate::{animation::{systems::map_atlas_layout, AnimatedEntity, AnimationIndices}, mouse::{
    get_mouse_animation, Mouse, MouseMovement, MovementState
}, MOUSE_SIZE};

use super::{key_just_pressed, key_pressed, GRAVITY};

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_query: Query<(
        &mut Transform, &mut Handle<Image>, &mut TextureAtlas, &mut AnimationIndices
    ), With<Mouse>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut mouse_movement: ResMut<MouseMovement>
) {
    if let Ok(
        (mut transform, mut image, mut atlas, mut indices)
    ) = mouse_query.get_single_mut() {
        let mut direction: Vec3 = Vec3::ZERO;
        let mut x: f32 = 0.0;
        let mut is_jump: bool = false;

        if key_just_pressed(&keyboard_input, KeyCode::Space) { is_jump = true; }
        if key_pressed(&keyboard_input, KeyCode::ArrowLeft) { x = -1.0; } 
        if key_pressed(&keyboard_input, KeyCode::ArrowRight) { x = 1.0; }

        let is_move: bool = x != 0.0;

        if mouse_movement.state == MovementState::Jump {
            let jump_power = GRAVITY * time.delta_seconds() * 3.0 - mouse_movement.jump;
            transform.translation.y += jump_power;
            if transform.translation.y < MOUSE_SIZE {
                transform.translation.y = MOUSE_SIZE;
                mouse_movement.set_state(MovementState::Idle);
            }
            mouse_movement.jump += time.delta_seconds() * 8.2;
            return;
        }

        if is_move {
            mouse_movement.set_state(MovementState::Move);
            direction += Vec3::new(x, 0.0, 0.0);
            direction = direction.normalize();
            
            if x < 0.0 {
                transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            } else {
                transform.rotation = Quat::default();
            }

            transform.translation += direction * mouse_movement.speed * time.delta_seconds();
        }
        if is_jump {
            mouse_movement.set_state(MovementState::Jump);
            mouse_movement.set_jump(1.0);
        } 

        let animated_entity: AnimatedEntity = get_mouse_animation(
            &mouse_movement.state,
            asset_server
        );
        *image = animated_entity.texture;
        *indices = animated_entity.animation_indices;
        atlas.layout = texture_atlas_layouts.add(map_atlas_layout(&animated_entity.sprite_layout));

        if !is_move && !is_jump { mouse_movement.set_state(MovementState::Idle); }
    }
}
