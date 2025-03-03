use bevy::{prelude::*, sprite::Anchor};

use crate::{
    actors::components::{Actor, Player},
    physics::components::{Collider, DragCurve, Sensor, Velocity},
};

pub struct ActorPart {
    pub name: String,
    pub entity: Entity,
}

pub struct PartBuilder {
    pub name: String,
    pub transform: Transform,
    pub image: Handle<Image>,
    pub color: Color,
    pub size: Vec2,
    pub anchor: Anchor,
    pub collider: Collider,
}

impl PartBuilder {
    pub fn new(name: impl ToString, image: Handle<Image>) -> Self {
        Self {
            image,
            name: name.to_string(),
            transform: Transform::IDENTITY,
            color: Color::default(),
            size: Vec2::splat(32.0),
            anchor: Anchor::Center,
            collider: Collider::Circle(16.0),
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }

    pub fn set_anchor(&mut self, anchor: Anchor) {
        self.anchor = anchor;
    }

    pub fn set_collider(&mut self, collider: Collider) {
        self.collider = collider;
    }

    pub fn build(&self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                Name::new(self.name.clone()),
                self.transform,
                Sprite {
                    image: self.image.clone(),
                    color: self.color,
                    custom_size: Some(self.size),
                    anchor: self.anchor,
                    ..Default::default()
                },
                self.collider,
            ))
            .id()
    }
}

pub struct ActorBuilder {
    pub transform: Transform,
    pub velocity: Velocity,
    pub drag: DragCurve,
    pub parts: Vec<PartBuilder>,
    pub is_player: bool,
}

impl ActorBuilder {
    pub fn new(transform: Transform) -> Self {
        Self {
            transform,
            parts: Vec::new(),
            velocity: Velocity::IDENTITY,
            is_player: false,
            drag: DragCurve::default(),
        }
    }

    pub fn add_part(&mut self, part: PartBuilder) {
        self.parts.push(part);
    }

    pub fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }

    pub fn set_drag(&mut self, drag: DragCurve) {
        self.drag = drag;
    }

    pub fn set_player(&mut self, is_player: bool) {
        self.is_player = is_player;
    }

    /// Build a heirarchy of entities representing the actor
    pub fn build(&self, commands: &mut Commands) {
        // Build root entity
        let root = commands
            .spawn((
                Name::new("Actor"),
                Actor,
                self.transform,
                Collider::circle(32.0),
                Sensor,
                self.velocity,
                self.drag.clone(),
            ))
            .id();

        if self.is_player {
            commands.entity(root).insert(Player);
        }

        // Create a child entity which will hold the graphic model/sprite
        let model = commands
            .spawn((Name::new("Model"), Transform::IDENTITY))
            .id();

        for builder in self.parts.iter() {
            let part = builder.build(commands);
            commands.entity(model).add_child(part);
        }

        commands.entity(root).add_child(model);
    }
}
