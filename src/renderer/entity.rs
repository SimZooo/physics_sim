use std::{any, collections::HashMap};

use macroquad::{
    color::{Color, GREEN},
    shapes::{draw_circle, draw_rectangle, draw_rectangle_lines},
    window::{screen_height, screen_width},
};

use crate::{
    math::math::Vec2f,
    physics::entities::physics_body::{BoundingBox, PhysicsBody, RigidBody},
};

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
        println!("{:?}", physics_body.position);
        Self {
            size,
            color,
            bounding_box: BoundingBox::new(
                physics_body.position.x.clone(),
                physics_body.position.y.clone(),
                size.x * 2.,
                size.y * 2.,
            ),
            physics_body,
            rigidbody,
            shape,
        }
    }

    fn render(&self, ppu: f32, debug: bool) {
        match self.shape {
            Shape::Circle => {
                draw_circle(
                    self.physics_body.position.x * ppu,
                    screen_height() - self.physics_body.position.y * ppu,
                    self.size.x,
                    self.color,
                );
                if debug {
                    draw_rectangle_lines(
                        self.physics_body.position.x * ppu - self.size.x,
                        screen_height() - self.physics_body.position.y * ppu - self.size.y,
                        self.size.x * 2.,
                        self.size.y * 2.,
                        2.,
                        GREEN,
                    );
                }
            }
            Shape::Rectangle => {
                draw_rectangle(
                    self.physics_body.position.x,
                    screen_height() - self.physics_body.position.y * ppu,
                    self.size.x,
                    self.size.y,
                    self.color,
                );
                if debug {
                    draw_rectangle_lines(
                        self.physics_body.position.x * ppu,
                        screen_height() - self.physics_body.position.y * ppu,
                        self.size.x,
                        self.size.y,
                        2.,
                        GREEN,
                    );
                }
            }
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

    pub fn render_all(&self, ppu: f32, debug: bool) {
        self.entities.iter().for_each(|(_, e)| e.render(ppu, debug));
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
