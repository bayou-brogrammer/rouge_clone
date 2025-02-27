use bevy::prelude::*;

use crate::actors::components::Actor;

pub struct ActorsPlugin;
impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Actor>();
    }
}
