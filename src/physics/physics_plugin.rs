use bevy::prelude::*;

use crate::physics::components::{Collider, Sensor, Velocity};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Collider>();
        app.register_type::<Sensor>();
        app.register_type::<Velocity>();
    }
}
