use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Copy, Deref, DerefMut)]
#[reflect(Component)]
#[require(Transform)]
pub struct Velocity {
    pub value: Vec2,
}

impl Default for Velocity {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Velocity {
    pub const IDENTITY: Self = Self { value: Vec2::ZERO };

    pub fn new(value: Vec2) -> Self {
        Self { value }
    }
}
