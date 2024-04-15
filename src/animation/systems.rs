use bevy::prelude::*;

use super::{
    AnimatedEntity, AnimationIndices, AnimationTimer, SpriteLayout
};

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index >= indices.last {
                indices.first
            } else {
                atlas.index + 1
            }
        }
    }
}

pub fn spawn_animated_entity(
    mut commands: Commands,
    animated_entity: AnimatedEntity,
    translation: Vec3,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    component: impl Component
) -> () {
    let layout: TextureAtlasLayout = map_atlas_layout(&animated_entity.sprite_layout);
    
    commands.spawn((
        SpriteSheetBundle {
            texture: animated_entity.get_texture().clone(),
            atlas: TextureAtlas {
                layout: texture_atlas_layouts.add(layout),
                index: animated_entity.get_animation_incides().first,
            },
            transform: Transform { 
                translation, 
                scale: Vec3 { x: 2.0, y: 2.0, z: 0.0 }, 
                ..Default::default() 
            },
            ..default()
        },
        animated_entity.get_animation_incides(),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        component
    ));
}

pub fn map_atlas_layout(sprite_layout: &SpriteLayout) -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(
        Vec2::new(
            sprite_layout.width,
            sprite_layout.height
        ),
        sprite_layout.columns, sprite_layout.rows, None, None
    )
}