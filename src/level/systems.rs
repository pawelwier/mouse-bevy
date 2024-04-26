use bevy::prelude::*;

use crate::mouse::{Mouse, MOUSE_SIZE};

use super::{LevelElement, StaticElement};

pub const BRICK_SIZE: f32 = 48.0;
pub const CHEESE_SIZE: (f32, f32) = (860.0, 661.0);
pub const CHEESE_SCALE: f32 = 0.13;

pub const BRICKS: [(f32, f32); 4] = [
    (50.0, BRICK_SIZE / 2.0),
    (180.0, BRICK_SIZE * 1.7),
    (310.0, BRICK_SIZE  * 2.5),
    (440.0, BRICK_SIZE  * 3.2),
];

#[derive(Component)]
pub struct Cheese {}

pub fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) -> () {
    for (x, y) in BRICKS {
        commands.spawn(
            (
                SpriteBundle {
                    texture: asset_server.load("sprites/brick.png"),
                    transform: Transform {
                        translation: Vec3 { x, y, z: 0.0 },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                LevelElement {},
                StaticElement {
                    size: (BRICK_SIZE, BRICK_SIZE)
                }
            )
        );
    };
    commands.spawn(
        (
            SpriteBundle {
                texture: asset_server.load("sprites/cheese.png"),
                transform: Transform {
                    translation: Vec3 { x: 570.0, y: 300.0, z: 0.0 },
                    scale: Vec3 { x: CHEESE_SCALE, y: CHEESE_SCALE, z: 0.0 },
                    ..Default::default()
                },
                ..Default::default()
            },
            LevelElement {},
            StaticElement {
                size: (CHEESE_SIZE.0, CHEESE_SIZE.1)
            },
            Cheese {}
        )
    );
}

pub fn grab_cheese(
    mut commands: Commands,
    mut mouse_query: Query<&mut Transform, With<Mouse>>,
    mut cheese_query: Query<(&Transform, Entity), (With<Cheese>, Without<Mouse>)>
) {
    let mouse = mouse_query.get_single_mut().unwrap();

    if let Ok((transform, entity)) = cheese_query.get_single_mut() {
        // TODO: make a generic clash fn
        if transform.translation.x < mouse.translation.x + MOUSE_SIZE
            && transform.translation.x + CHEESE_SIZE.0 * CHEESE_SCALE > mouse.translation.x {
                commands.entity(entity).despawn();
            }
    } else { 
        return;
    }
}