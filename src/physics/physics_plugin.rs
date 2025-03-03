use bevy::prelude::*;

use crate::physics::{
    components::{Collider, DragCurve, Sensor, Velocity},
    data::{CustomCurve, Interpolation},
    systems::{apply_drag, apply_velocity},
};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Collider>();
        app.register_type::<CustomCurve>();
        app.register_type::<DragCurve>();
        app.register_type::<Interpolation>();
        app.register_type::<Sensor>();
        app.register_type::<Velocity>();

        app.add_systems(Update, (apply_drag, apply_velocity).chain());
    }
}
