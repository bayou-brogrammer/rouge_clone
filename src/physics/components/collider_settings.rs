use bevy::prelude::*;

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct ColliderSettings {
    pub signal: bool,
}

impl Default for ColliderSettings {
    fn default() -> Self {
        Self { signal: false }
    }
}
