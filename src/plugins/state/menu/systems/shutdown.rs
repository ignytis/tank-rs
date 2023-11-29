use bevy::prelude::*;

use crate::plugins::state::menu::components;

pub fn shutdown(
    mut commands: Commands,
    node_bundle_query: Query<Entity, With<components::MenuBlock>>,
) {
    for entity in node_bundle_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}