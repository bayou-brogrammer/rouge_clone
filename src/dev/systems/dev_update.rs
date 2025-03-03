use bevy::prelude::*;

use crate::{actors::components::Actor, physics::components::Velocity};

/// Temporary system for quick prototyping
pub fn dev_update(
    time: Res<Time>,
    mut q_actors: Query<(&mut Transform, &Velocity), With<Actor>>
) {
    let dt = time.delta_secs();

    for (mut transform, velocity) in q_actors.iter_mut() {
        // Apply velocity to position
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }
}
