use bevy::prelude::*;

pub const BRICK_SIZE: f32 = 48.0;

pub const BRICKS: [(f32, f32); 3] = [
    (50.0, BRICK_SIZE / 2.0),
    (200.0, BRICK_SIZE / 2.0),
    (500.0, BRICK_SIZE  * 2.0)
];

pub fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) -> () {
    for (x, y) in BRICKS {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("sprites/brick.png"),
            transform: Transform {
                translation: Vec3 { x, y, z: 0.0 },
                ..Default::default()
            },
            ..Default::default()
        });   
    }
}