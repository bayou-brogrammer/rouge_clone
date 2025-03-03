use bevy::prelude::*;

use crate::physics::{components::Velocity, data::CustomCurve};

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
#[require(Velocity)]
pub struct DragCurve {
    pub curve: CustomCurve,
    pub elapsed_time: f32,
}

impl Default for DragCurve {
    fn default() -> Self {
        Self {
            curve: CustomCurve::default(),
            elapsed_time: 0.0,
        }
    }
}

impl DragCurve {
    pub fn new(curve: CustomCurve) -> Self {
        Self {
            curve,
            elapsed_time: 0.0,
        }
    }

    pub fn coefficient(&self) -> f32 {
        self.curve.sample_unchecked(self.elapsed_time)
    }
}
