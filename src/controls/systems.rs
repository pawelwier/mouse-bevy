use bevy::{prelude::*, window::PrimaryWindow};

use crate::{animation::{systems::map_atlas_layout, AnimatedEntity}, level::{StaticElement}, mouse::{
    get_mouse_animation, Direction, Mouse, MouseMovement, MovementState, MOUSE_JUMP, MOUSE_MARGIN, MOUSE_SCALE, MOUSE_SIZE
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

fn move_mouse(
    mouse_movement: &mut MouseMovement,
    window: &Window,
    move_direction: f32,
    transform: &mut Transform,
    time: &Res<Time>,
    is_vertical: bool
) -> () {
    if !limit_player_movement(
        window, move_direction, transform.translation.x
    ) { 
        let movement: f32 = move_direction * mouse_movement.speed * time.delta_seconds();
        if is_vertical {
            transform.translation.y += movement;
        } else {
            transform.translation.x += movement;
        }
    } else {
        mouse_movement.direction = Direction::Still;
    }
}

fn set_atlas_layout(
    mouse_movement: ResMut<MouseMovement>,
    asset_server: Res<AssetServer>,
    image: &mut Handle<Image>, 
    atlas: &mut TextureAtlas,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) -> () {
    let animated_entity: AnimatedEntity = get_mouse_animation(
        &mouse_movement.state,
        asset_server
    );
    *image = animated_entity.texture;
    atlas.layout = texture_atlas_layouts.add(map_atlas_layout(&animated_entity.sprite_layout));
}

pub fn player_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_query: Query<(
        &mut Transform, &mut Handle<Image>, &mut TextureAtlas
    ), With<Mouse>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut mouse_movement: ResMut<MouseMovement>
) {
    if let Ok(
        (mut transform, mut image, mut atlas)
    ) = mouse_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let mut direction: Vec3 = Vec3::ZERO;
        let mut x: f32 = 0.0;
        let mut is_jump: bool = false;

        if key_just_pressed(&keyboard_input, KeyCode::ArrowUp) { is_jump = true; }
        if key_pressed(&keyboard_input, KeyCode::ArrowLeft) { x = -1.0; } 
        if key_pressed(&keyboard_input, KeyCode::ArrowRight) { x = 1.0; }

        let is_move: bool = x != 0.0;

        if mouse_movement.is_jumping() {
            let jump_power = GRAVITY * time.delta_seconds() * 3.0 - mouse_movement.jump;
            transform.translation.y += jump_power;
            if jump_power < 0.0 { mouse_movement.set_state(MovementState::Fall); }

            if mouse_movement.is_moving() {
                let direction: f32 = if mouse_movement.direction == Direction::Left { -1.0 } else { 1.0 };
                move_mouse(
                    &mut mouse_movement,
                    &window,
                    direction * 0.6,
                    &mut transform,
                    &time,
                    false
                )
            }

            if transform.translation.y < MOUSE_SIZE - MOUSE_MARGIN {
                transform.translation.y = MOUSE_SIZE - MOUSE_MARGIN;
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

        if is_move && !is_jump {
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

            move_mouse(
                &mut mouse_movement,
                &window,
                direction.x,
                &mut transform,
                &time,
                false
            )
        }
        
        if is_jump {
            mouse_movement.set_state(MovementState::Jump);
            mouse_movement.set_jump(1.0);
        } 

        if !is_move && !is_jump { 
            mouse_movement.set_state(MovementState::Idle); 
            mouse_movement.set_direction(Direction::Still);
        }

        set_atlas_layout(
            mouse_movement,
            asset_server,
            &mut image,
            &mut atlas,
            texture_atlas_layouts
        );
    }
}



pub fn check_mouse_on_el(
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut mouse_query: Query<&mut Transform, With<Mouse>>,
    mut mouse_movement: ResMut<MouseMovement>,
    elements_query: Query<(&Transform, &StaticElement), (With<StaticElement>, Without<Mouse>)>
) -> () {
    let window = window_query.get_single().unwrap();

    let mut mouse = mouse_query.get_single_mut().unwrap();
    let mouse_bottom: f32 = mouse.translation.y - MOUSE_MARGIN;

    /* Doesn't fall if mouse is on the ground */
    if mouse_bottom < 1.0 || mouse_movement.is_jumping() { return; }

    let mut mouse_fall = true;

    for (el_transform, el) in elements_query.iter() {
        let (el_width, el_height) = el.size;
        let mouse_above_el: bool = el_transform.translation.x < mouse.translation.x + MOUSE_SIZE
            && el_transform.translation.x + el_width > mouse.translation.x;

            if mouse_above_el && mouse.translation.y == el_transform.translation.y + el_height / 2.0 + MOUSE_MARGIN {
                mouse_fall = false
            }
        }
    
    if mouse_fall {
        let mut direction: Vec3 = Vec3::new(0.0, -1.0, 0.0);
        direction = direction.normalize();

        move_mouse(
            &mut mouse_movement,
            &window,
            direction.y,
            &mut mouse,
            &time,
            true
        )
    }
}


pub fn check_jump_on_object(
    mut mouse_query: Query<&mut Transform, With<Mouse>>,
    mut mouse_movement: ResMut<MouseMovement>,
    elements_query: Query<(&Transform, &StaticElement), (With<StaticElement>, Without<Mouse>)>
) -> () {
    let mut mouse = mouse_query.get_single_mut().unwrap();
    let mouse_bottom: f32 = mouse.translation.y - MOUSE_MARGIN;

    // TODO: make a generic clash fn
    for (el_transform, el) in elements_query.iter() {
        let (el_width, el_height) = el.size;
        let el_top: f32 = el_transform.translation.y / 2.0 + el_height;
        let mouse_above_el: bool = el_transform.translation.x < mouse.translation.x + MOUSE_SIZE
            && el_transform.translation.x + el_width > mouse.translation.x;
        let fall_on_object: bool = mouse_movement.state != MovementState::Jump
            && mouse_bottom > el_top
            // TODO: come up with another way to check
            //  if mouse jumps eg. vertically on top of element
            && mouse_bottom - el_top < 7.0 
            && mouse_above_el;
            
        
        if fall_on_object {
            mouse.translation.y = el_transform.translation.y + el_height / 2.0 + MOUSE_MARGIN;
            mouse_movement.set_state(MovementState::Idle);
        }
    }
}