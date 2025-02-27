use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Copy)]
#[reflect(Component)]
#[require(Transform)]
pub enum Collider {
    Circle(f32),
    Rectangle(f32, f32),
}

impl Collider {
    pub fn circle(radius: f32) -> Self {
        Self::Circle(radius)
    }

    pub fn rectangle(width: f32, height: f32) -> Self {
        Self::Rectangle(width, height)
    }
}
