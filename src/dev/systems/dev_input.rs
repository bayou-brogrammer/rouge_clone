use bevy::prelude::*;

use crate::{actors::components::Actor, physics::components::Velocity};

pub fn dev_input(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut query: Query<&mut Velocity, With<Actor>>,
) {
  for mut velocity in query.iter_mut() {
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

      // Normalize the direction vector to ensure consistent speed in all directions
      if direction != Vec2::ZERO {
          direction = direction.normalize();
      }

      // Set the velocity (you can adjust the speed multiplier as needed)
      velocity.set(direction * 200.0);
  }
}
