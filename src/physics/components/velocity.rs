use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Copy, Deref, DerefMut)]
#[reflect(Component)]
#[require(Transform)]
pub struct Velocity(Vec2);

impl Velocity {
    pub const IDENTITY: Self = Self(Vec2::ZERO);

    pub fn new(value: Vec2) -> Self {
        Self(value)
    }

    pub fn set(&mut self, value: Vec2) {
        self.0 = value;
    }
}
