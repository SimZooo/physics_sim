use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct TransformComponent {
    pub position: Position,
}

pub struct ComponentManager {
    pub storages: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentManager {
    pub fn init() -> Self {
        Self {
            storages: HashMap::new(),
        }
    }
}
