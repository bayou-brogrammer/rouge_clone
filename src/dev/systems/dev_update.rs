use bevy::prelude::*;

use crate::actors::components::Actor;

/// Temporary system for quick prototyping
pub fn dev_update(time: Res<Time>, mut q_actors: Query<&mut Transform, With<Actor>>) {
    for mut transform in q_actors.iter_mut() {
        transform.translation.x += 32.0 * time.delta_secs();
    }
}
