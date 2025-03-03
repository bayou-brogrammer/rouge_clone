use bevy::prelude::*;

use crate::physics::components::{DragCurve, Velocity};

pub fn apply_drag(mut q_velocity: Query<(&mut Velocity, &DragCurve)>) {
    for (mut velocity, drag) in q_velocity.iter_mut() {
        let coefficient = drag.coefficient();
        velocity.value *= 1.0 - coefficient;
    }
}
