use bevy::prelude::*;

use crate::physics::components::{Collider, ColliderSettings};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ColliderSettings>();
        app.register_type::<Collider>();
    }
}
