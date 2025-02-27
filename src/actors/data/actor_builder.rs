use bevy::{prelude::*, sprite::Anchor};

use crate::{
    actors::components::Actor,
    physics::components::{Collider, ColliderSettings},
};

pub struct ActorEntities {
    pub root: Entity,
    pub model: Entity,
    pub colliders: Entity,
}

pub struct ActorBuilder {
    pub color: Color,
    pub position: (f32, f32),
}

impl ActorBuilder {
    pub fn new(position: (f32, f32), color: impl Into<Color>) -> Self {
        Self {
            position,
            color: color.into(),
        }
    }

    /// Build a heirarchy of entities representing the actor
    /// TODO: Maybe the Model images should hold their own physical colliders? This would allow the graphics
    /// and collider to share the same transform guarenteeing that they are always in sync
    pub fn build(&self, commands: &mut Commands, asset_server: &AssetServer) -> ActorEntities {
        // Build root entity
        let root = commands
            .spawn((
                Name::new("Actor"),
                Actor,
                Transform::from_xyz(self.position.0, self.position.1, 0.0),
            ))
            .id();

        // Create a child entity which will hold the graphic model/sprite
        let model = commands
            .spawn((
                Name::new("Model"),
                Sprite {
                    image: asset_server.load("circle_32x32.png"),
                    // texture_atlas: todo!(),
                    color: self.color,
                    custom_size: Some(Vec2::splat(32.0)),
                    anchor: Anchor::Center,
                    ..Default::default()
                },
            ))
            .id();

        // Create a child which will hold the colliders
        let colliders = commands
            .spawn((Name::new("Colliders"), Transform::IDENTITY))
            .id();

        let physical_collider = commands
            .spawn((Name::new("Physical Collider"), Collider::circle(16.0)))
            .id();

        let interact_collider = commands
            .spawn((
                Name::new("Interact Collider"),
                Collider::circle(32.0),
                ColliderSettings { signal: true },
            ))
            .id();

        commands
            .entity(colliders)
            .add_child(physical_collider)
            .add_child(interact_collider);

        commands.entity(root).add_child(model).add_child(colliders);

        ActorEntities {
            root,
            model,
            colliders,
        }
    }
}
