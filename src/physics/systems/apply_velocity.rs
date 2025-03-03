use bevy::prelude::*;

use crate::physics::components::Velocity;

pub fn apply_velocity(time: Res<Time>, mut q_velocity: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in q_velocity.iter_mut() {
        transform.translation += Vec3::new(
            velocity.value.x * time.delta_secs(),
            velocity.value.y * time.delta_secs(),
            0.0,
        );
    }
}
