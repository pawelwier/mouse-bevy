use bevy::{
    prelude::*, window::{
        PrimaryWindow, WindowResolution
    }
};

use animation::{
    systems::{
        animate_sprite, spawn_animated_entity
    },
    AnimatedEntity, AnimationIndices, SpriteLayout
};
use controls::player_movement;
use mouse::{Mouse, MouseMovement};

mod mouse;
mod animation;
mod controls;

const MOUSE_SIZE: f32 = 64.0;
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) -> () {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..Default::default()
        }
    );
}

fn spawn_mouse(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let window = window_query.get_single().unwrap();
    let texture = asset_server.load("sprites/mouse_idle.png");

    spawn_animated_entity(
        commands,
        AnimatedEntity {
            texture,
            animation_indices: AnimationIndices { first: 0, last: 5 },
            sprite_layout: SpriteLayout {
                columns: 6,
                rows: 1,
                width: MOUSE_SIZE,
                height: MOUSE_SIZE
            }
        },
        Vec3 { 
            x: window.width() / 2.0, y: MOUSE_SIZE, z: 0.0 
        },
        texture_atlas_layouts,
        Mouse {}
    );
}

// fn handle_mouse_move(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     asset_server: Res<AssetServer>,
//     mut mouse_query: Query<&mut Handle<Image>, With<Mouse>>
// ) -> () {
//     if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
//         if let Ok(handle) = mouse_query.get_single_mut() {
//             let texture = asset_server.load("sprites/mouse_idle.png");

//             handle = *texture.clone();
//         }
//     }
// }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(
                WindowPlugin {
                    primary_window: Some(
                        Window {
                            title: "MOUSE GAME".to_string(),
                            resizable: false,
                            resolution: WindowResolution::new(
                                WINDOW_WIDTH,
                                WINDOW_HEIGHT
                            ),
                            ..Default::default()
                        }
                    ),
                    ..Default::default()
                },
            )
            .set(ImagePlugin::default_nearest())
        )
        .init_resource::<MouseMovement>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_mouse)
        .add_systems(Update, animate_sprite)
        .add_systems(Update, player_movement)
        .run();
}
