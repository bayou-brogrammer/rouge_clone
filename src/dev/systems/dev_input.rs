use bevy::prelude::*;

use crate::{
    actors::components::Player,
    physics::components::{DragCurve, Velocity},
};

const DISTANCE: f32 = 32.0; // The distance of 1 unit (because we are using a 16 radius circle)
const SPEED: f32 = 1.0; // Acceleration speed

pub fn dev_input(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut DragCurve), With<Player>>,
) {
    for (mut velocity, mut drag) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        if direction != Vec2::ZERO {
            // Normalize the direction vector to ensure consistent speed in all directions
            direction = direction.normalize();

            // TODO: Change this to use an acceleration curve
            velocity.value += direction * DISTANCE * SPEED;
            // We are adding more velocity this frame, so reset the drag timer
            drag.elapsed_time = 0.0
        } else {
            // We are not adding more velocity this frame, so increment the drag timer
            drag.elapsed_time += time.delta_secs();
        }
    }
}
