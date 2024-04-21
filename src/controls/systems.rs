use bevy::{prelude::*, window::PrimaryWindow};

use crate::{animation::{systems::map_atlas_layout, AnimatedEntity, AnimationIndices}, mouse::{
    get_mouse_animation, Direction, Mouse, MouseMovement, MovementState, MOUSE_BOTTOM_MARGIN, MOUSE_JUMP, MOUSE_SCALE, MOUSE_SIZE
}};

use super::{key_just_pressed, key_pressed, GRAVITY};

fn limit_player_movement(
    window: &Window,
    move_direction: f32,
    mouse_position: f32

) -> bool {
    let left_border: bool = mouse_position <= MOUSE_SIZE / MOUSE_SCALE / 2.0 && move_direction == -1.0;
    let right_border: bool = mouse_position >= window.width() - MOUSE_SIZE / MOUSE_SCALE / 2.0 && move_direction == 1.0;
    left_border || right_border
}

pub fn player_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_query: Query<(
        &mut Transform, &mut Handle<Image>, &mut TextureAtlas, &mut AnimationIndices
    ), With<Mouse>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut mouse_movement: ResMut<MouseMovement>
) {
    /* TODO: fix sprites */
    if let Ok(
        (mut transform, mut image, mut atlas, mut indices)
    ) = mouse_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

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
            if mouse_movement.is_moving() {
                let direction: f32 = if mouse_movement.direction == Direction::Left { -1.0 } else { 1.0 };
                if !limit_player_movement(
                    window, x, transform.translation.x
                ) { 
                    transform.translation.x += 2.3 * direction;
                } else {
                    mouse_movement.direction = Direction::Still;
                }
             }
            if transform.translation.y < MOUSE_SIZE - MOUSE_BOTTOM_MARGIN {
                transform.translation.y = MOUSE_SIZE - MOUSE_BOTTOM_MARGIN;
                let new_state:MovementState = if mouse_movement.is_moving() {
                    MovementState::Move
                } else {
                    MovementState::Idle
                };
                mouse_movement.set_state(new_state);
                return;
            }
            mouse_movement.jump += time.delta_seconds() * MOUSE_JUMP;
            return;
        }

        if is_move {
            mouse_movement.set_state(MovementState::Move);
            direction += Vec3::new(x, 0.0, 0.0);
            direction = direction.normalize();
            
            if x < 0.0 {
                transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
                mouse_movement.set_direction(Direction::Left);
            } else {
                transform.rotation = Quat::default();
                mouse_movement.set_direction(Direction::Right);

            }

            if !limit_player_movement(
                window, x, transform.translation.x
            ) { transform.translation += direction * mouse_movement.speed * time.delta_seconds(); }
        }
        
        if is_jump {
            mouse_movement.set_state(MovementState::Jump);
            mouse_movement.set_jump(1.0);
        } 

        if !is_move && !is_jump && mouse_movement.state != MovementState::Jump { 
            mouse_movement.set_state(MovementState::Idle); 
            mouse_movement.set_direction(Direction::Still);
        }

        let animated_entity: AnimatedEntity = get_mouse_animation(
            &mouse_movement.state,
            asset_server
        );
        *image = animated_entity.texture;
        *indices = animated_entity.animation_indices;
        atlas.layout = texture_atlas_layouts.add(map_atlas_layout(&animated_entity.sprite_layout));
    }
}