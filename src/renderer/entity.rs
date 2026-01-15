use std::{any, collections::HashMap};

use macroquad::{color::Color, shapes::draw_circle};

use crate::physics::entities::physics_body::PhysicsBody;

pub struct Entity {
    pub size: f32,
    pub color: Color,
    pub physics_body: Option<PhysicsBody>,
}

impl Entity {
    pub fn new(size: f32, color: Color, physics_body: Option<PhysicsBody>) -> Self {
        Self {
            size,
            color,
            physics_body,
        }
    }

    fn render(&self) {
        if let Some(physics_body) = &self.physics_body {
            draw_circle(
                physics_body.position.x,
                physics_body.position.y,
                self.size,
                self.color,
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

    pub fn render_all(&self) {
        self.entities.iter().for_each(|(_, e)| e.render());
    }

    pub fn get_entity(&self, id: &EntityId) -> Option<&Entity> {
        self.entities.get(id)
    }

    pub fn get_entity_mut(&mut self, id: &EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(id)
    }
}
