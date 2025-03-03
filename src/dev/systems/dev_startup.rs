use bevy::{color::palettes::css, prelude::*, sprite::Anchor};

use crate::{
    actors::data::{ActorBuilder, PartBuilder},
    physics::{
        components::{Collider, DragCurve, Velocity},
        data::{CustomCurve, Interpolation},
    },
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
    let curve = CustomCurve::new(0.0, Interpolation::Linear)
        .with_point(0.25, 0.0, Interpolation::LogOut)
        .with_point(0.5, 0.05, Interpolation::LogIn)
        .with_point(1.0, 1.0, Interpolation::Linear);
    let drag_curve = DragCurve::new(curve);
    player_builder.set_drag(drag_curve);
    player_builder.set_player(true);

    player_builder.build(&mut commands);
}
