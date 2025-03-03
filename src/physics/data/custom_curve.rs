use std::collections::BTreeMap;

use bevy::prelude::*;
use ordered_float::OrderedFloat;

use crate::physics::data::Interpolation;

#[derive(Reflect, Clone)]
pub struct CustomCurve {
    #[reflect(ignore)]
    pub points: BTreeMap<OrderedFloat<f32>, (f32, Interpolation)>,
}

impl Default for CustomCurve {
    fn default() -> Self {
        Self::new(0.0, Interpolation::Linear)
    }
}

impl CustomCurve {
    /// Creates a new curve beginning at Time = 0.0, with the given value and future interpolation
    pub fn new(value: f32, interpolation: Interpolation) -> Self {
        let mut points = BTreeMap::new();
        points.insert(OrderedFloat(0.0), (value, interpolation));
        Self { points }
    }

    /// Adds a point to the curve at the given time, with the given value and future interpolation
    pub fn with_point(mut self, t: f32, value: f32, interpolation: Interpolation) -> Self {
        self.points.insert(OrderedFloat(t), (value, interpolation));
        self
    }

    /// Adds a point to the curve at the given time, with the given value and future interpolation
    pub fn add_point(&mut self, t: f32, value: f32, interpolation: Interpolation) {
        self.points.insert(OrderedFloat(t), (value, interpolation));
    }
}

impl Curve<f32> for CustomCurve {
    fn domain(&self) -> Interval {
        Interval::EVERYWHERE
    }

    fn sample_unchecked(&self, t: f32) -> f32 {
        let t_key = OrderedFloat(t);

        let lower = self.points.range(..=t_key).next_back();
        let upper = self.points.range(t_key..).next();

        match (lower, upper) {
            (Some((&t0, &(v0, interpolation))), Some((&t1, &(v1, _i)))) if t0 != t1 => {
                let alpha = (t - t0.0) / (t1.0 - t0.0);

                match interpolation {
                    Interpolation::Linear => v0 + alpha * (v1 - v0),
                    Interpolation::LogIn => v0 + (alpha * alpha) * (v1 - v0), // Ease in
                    Interpolation::LogOut => v0 + (1.0 - (1.0 - alpha).powi(2)) * (v1 - v0), // Ease out
                }
            }
            (Some((&_t0, &(v0, _i))), None) => v0,
            _ => 0.0,
        }
    }
}
