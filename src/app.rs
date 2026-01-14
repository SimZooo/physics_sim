use macroquad::{
    color::RED,
    window::{clear_background, next_frame},
};

use crate::entity::EntityManager;

pub struct AppContext {
    pub entity_manager: EntityManager,
}

pub struct App<S> {
    pub app_context: AppContext,
    pub systems: Vec<fn(&mut AppContext, f32, &mut S)>,
    pub state: S,
}

impl<S> App<S> {
    pub fn new(state: S) -> Self {
        Self {
            app_context: AppContext {
                entity_manager: EntityManager::init(),
            },
            systems: vec![],
            state,
        }
    }

    pub fn add_system_function(&mut self, function: fn(&mut AppContext, f32, &mut S)) {
        self.systems.push(function);
    }

    pub async fn run(&mut self) {
        loop {
            let dt = macroquad::time::get_frame_time();
            for i in 0..self.systems.len() {
                let system = self.systems[i];
                (system)(&mut self.app_context, dt, &mut self.state);
            }

            clear_background(RED);
            self.app_context.entity_manager.render_all();
            next_frame().await;
        }
    }
}
