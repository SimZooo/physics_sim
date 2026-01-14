use std::{any, collections::HashMap};

pub trait Entity {
    fn render(&self) {}
    fn as_any(&mut self) -> &mut dyn any::Any;
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct EntityId(pub usize);

pub struct EntityManager {
    pub entities: HashMap<EntityId, Box<dyn Entity>>,
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

    pub fn add(&mut self, entity: Box<dyn Entity>) -> EntityId {
        let entity_id = self.new_entity_id();
        self.entities.insert(entity_id, entity);
        entity_id
    }

    pub fn render_all(&self) {
        self.entities.iter().for_each(|(_, e)| e.render());
    }

    pub fn get_entity(&self, id: &EntityId) -> Option<&Box<dyn Entity>> {
        self.entities.get(id)
    }

    pub fn get_entity_mut(&mut self, id: &EntityId) -> Option<&mut Box<dyn Entity>> {
        self.entities.get_mut(id)
    }
}
