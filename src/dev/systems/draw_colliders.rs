use bevy::prelude::*;

use crate::physics::components::{Collider, Sensor};

const PHYSICAL_COLOR: Srgba = Srgba::new(0.0, 1.0, 0.0, 1.0);
const SIGNAL_COLOR: Srgba = Srgba::new(0.0, 0.0, 1.0, 1.0);

/// Draw all colliders using gizmos.
pub fn draw_colliders(
    q_colliders: Query<(&GlobalTransform, &Collider, Option<&Sensor>)>,
    mut gizmos: Gizmos,
) {
    for (transform, collider, maybe_settings) in q_colliders.iter() {
        let srgba = if maybe_settings.is_some() {
            SIGNAL_COLOR
        } else {
            PHYSICAL_COLOR
        };

        match collider {
            Collider::Circle(radius) => {
                gizmos.circle_2d(
                    transform.compute_transform().translation.truncate(),
                    *radius,
                    Color::Srgba(srgba),
                );
            }
            Collider::Rectangle(width, height) => {
                gizmos.rect_2d(
                    transform.compute_transform().translation.truncate(),
                    Vec2::new(*width, *height),
                    Color::Srgba(srgba),
                );
            }
        }
    }
}
