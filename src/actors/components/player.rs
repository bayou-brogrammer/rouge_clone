use bevy::prelude::*;

use crate::actors::components::Actor;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Actor)]
pub struct Player;
