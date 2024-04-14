use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution}
};

use animation::{
    systems::animate_sprite,
    AnimationIndices,
    AnimationTimer
};

mod mouse;
mod animation;

const MOUSE_SIZE: f32 = 64.0;
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..Default::default()
        }
    );
}

fn spawn_mouse(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let window = window_query.get_single().unwrap();
    let texture = asset_server.load("sprites/mouse_idle.png");
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(
            MOUSE_SIZE,
            MOUSE_SIZE
        ),
        6, 1, None, None
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 5 };
    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform { 
                translation: Vec3 { 
                    x: window.width() / 2.0, y: MOUSE_SIZE, z: 0.0 
                }, 
                scale: Vec3 { x: 2.0, y: 2.0, z: 0.0 }, 
                ..Default::default() 
            },
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
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
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_mouse)
        .add_systems(Update, animate_sprite)
        .run();
}
