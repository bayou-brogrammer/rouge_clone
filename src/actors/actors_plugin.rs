use bevy::prelude::*;

use crate::actors::components::{Actor, Player};

pub struct ActorsPlugin;
impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Actor>();
        app.register_type::<Player>();
    }
}
