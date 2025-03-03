use bevy::{color::palettes::css, prelude::*, sprite::Anchor};

use crate::{
    actors::data::{ActorBuilder, PartBuilder},
    physics::components::{Collider, Velocity},
};

/// Temporary system for quick prototyping
pub fn dev_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let mut player_builder = ActorBuilder::new(Transform::IDENTITY);
    let mut body = PartBuilder::new("Body", asset_server.load("circle_32x32.png"));
    body.set_color(Color::Srgba(css::YELLOW_GREEN));
    body.set_size(Vec2::splat(32.0));
    body.set_collider(Collider::Circle(16.0));
    body.set_anchor(Anchor::Center);
    player_builder.add_part(body);
    player_builder.set_velocity(Velocity::IDENTITY);

    player_builder.build(&mut commands);
}
