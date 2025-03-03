use bevy::prelude::*;

#[derive(Reflect, Default, Clone, Copy)]
pub enum Interpolation {
    #[default]
    Linear,
    LogIn,
    LogOut,
}
