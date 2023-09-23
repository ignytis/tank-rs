use bevy::prelude::*;

use crate::components::animation::{AnimationData, AnimationTimer};

pub fn animate_sprite(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &AnimationData,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (entity, indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if !timer.just_finished() {
            continue
        }

        if sprite.index == indices.last && indices.delete_after_last_frame {
            commands.entity(entity).despawn();
            continue
        }

        sprite.index = if sprite.index == indices.last {
            indices.first
        } else {
            sprite.index + 1
        };

    }
}