use bevy::{color::palettes::css, prelude::*};

use crate::actors::data::ActorBuilder;

/// Temporary system for quick prototyping
pub fn dev_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    ActorBuilder::new((0.0, 0.0), Color::Srgba(css::YELLOW_GREEN))
        .build(&mut commands, &asset_server);
}
