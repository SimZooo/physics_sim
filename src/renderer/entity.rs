use std::{any, collections::HashMap};

use macroquad::{
    color::{Color, GREEN, PURPLE, RED},
    miniquad::gl::GL_RENDERBUFFER,
    shapes::{draw_circle, draw_line, draw_rectangle, draw_rectangle_lines},
    window::{screen_height, screen_width},
};

use crate::{
    math::math::Vec2f,
    physics::entities::physics_body::{BoundingBox, PhysicsBody, RigidBody},
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Shape {
    Circle,
    Rectangle,
}

pub struct Entity {
    pub size: Vec2f,
    pub color: Color,
    pub physics_body: PhysicsBody,
    pub bounding_box: BoundingBox,
    pub rigidbody: RigidBody,
    pub shape: Shape,
}

impl Entity {
    pub fn new(
        size: Vec2f,
        color: Color,
        physics_body: PhysicsBody,
        shape: Shape,
        rigidbody: RigidBody,
    ) -> Self {
        let bb = match shape {
            Shape::Circle => {
                // Circle is rendered at center position
                BoundingBox::new(
                    physics_body.position.x.clone(),
                    physics_body.position.y.clone(),
                    size.x * 2.,
                    size.y * 2.,
                )
            }
            Shape::Rectangle => {
                // Rectangle is rendered with center offset, so bounding box center is at position
                BoundingBox::new(
                    physics_body.position.x.clone(),
                    physics_body.position.y.clone(),
                    size.x,
                    size.y,
                )
            }
        };
        Self {
            size,
            color,
            physics_body,
            bounding_box: bb,
            rigidbody,
            shape,
        }
    }

    fn render(&self, physics_dimensions: Vec2f, debug: bool, forces: bool, com: bool) {
        let mut pixel_coords = self.physics_body.position;
        pixel_coords.x = pixel_coords.x / physics_dimensions.x * screen_width();
        pixel_coords.y =
            screen_height() - (pixel_coords.y / physics_dimensions.y * screen_height());

        let pixel_size = Vec2f::new(
            self.size.x * (screen_width() / physics_dimensions.x),
            self.size.y * (screen_height() / physics_dimensions.y),
        );

        let bb_size = Vec2f::new(
            self.bounding_box.w * (screen_width() / physics_dimensions.x),
            self.bounding_box.h * (screen_height() / physics_dimensions.y),
        );
        match self.shape {
            Shape::Circle => {
                draw_circle(pixel_coords.x, pixel_coords.y, pixel_size.x, self.color);
                if debug {
                    draw_rectangle_lines(
                        pixel_coords.x - pixel_size.x,
                        pixel_coords.y - pixel_size.y,
                        bb_size.x,
                        bb_size.y,
                        2.,
                        GREEN,
                    );
                }
            }
            Shape::Rectangle => {
                draw_rectangle(
                    pixel_coords.x - pixel_size.x / 2.,
                    pixel_coords.y - pixel_size.y / 2.,
                    pixel_size.x,
                    pixel_size.y,
                    self.color,
                );
                if debug {
                    draw_rectangle_lines(
                        pixel_coords.x - pixel_size.x / 2.,
                        pixel_coords.y - pixel_size.y / 2.,
                        bb_size.x,
                        bb_size.y,
                        2.,
                        GREEN,
                    );
                }
            }
        }

        if com {
            draw_circle(pixel_coords.x, pixel_coords.y, 5., PURPLE);
        }

        if forces {
            draw_line(
                pixel_coords.x,
                pixel_coords.y,
                pixel_coords.x - self.physics_body.velocity.x * 10.,
                pixel_coords.y - self.physics_body.velocity.y * 10.,
                2.,
                RED,
            );
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct EntityId(pub usize);

pub struct EntityManager {
    pub entities: HashMap<EntityId, Entity>,
    curr_id: usize,
}

impl EntityManager {
    pub fn init() -> Self {
        Self {
            entities: HashMap::new(),
            curr_id: 0,
        }
    }

    fn new_entity_id(&mut self) -> EntityId {
        let e_id = EntityId(self.curr_id);
        self.curr_id += 1;
        e_id
    }

    pub fn add(&mut self, entity: Entity) -> EntityId {
        let entity_id = self.new_entity_id();
        self.entities.insert(entity_id, entity);
        entity_id
    }

    pub fn render_all(&self, physics_dimensions: Vec2f, debug: bool, forces: bool, com: bool) {
        self.entities
            .iter()
            .for_each(|(_, e)| e.render(physics_dimensions, debug, forces, com));
    }

    pub fn get_entity(&self, id: &EntityId) -> Option<&Entity> {
        self.entities.get(id)
    }

    pub fn get_entity_mut(&mut self, id: &EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(id)
    }

    pub fn clear(&mut self) {
        self.entities = HashMap::new();
    }
}
