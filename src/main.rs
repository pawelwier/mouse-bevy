use bevy::{
    prelude::*, window::{
        PrimaryWindow, WindowResolution
    }
};

use animation::
    systems::{
        animate_sprites, spawn_animated_entity
    };
use controls::systems::player_movement;
use level::systems::spawn_level;
use mouse::{
    get_mouse_animation, Mouse, MouseMovement, MovementState, MOUSE_BOTTOM_MARGIN, MOUSE_SIZE
};

mod mouse;
mod animation;
mod controls;
mod level;

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

    spawn_animated_entity(
        commands,
        get_mouse_animation(
            &MovementState::Idle,
            asset_server
        ),
        Vec3 { 
            x: window.width() / 2.0, y: MOUSE_SIZE - MOUSE_BOTTOM_MARGIN, z: 0.0 
        },
        texture_atlas_layouts,
        Mouse {}
    );
}

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
        .add_systems(Startup, spawn_level)
        .add_systems(Update, animate_sprites)
        .add_systems(Update, player_movement)
        .run();
}
