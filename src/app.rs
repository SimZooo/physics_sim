use macroquad::{
    color::RED,
    input::{MouseButton, is_mouse_button_down, mouse_position},
    miniquad::window::{screen_size, set_window_size},
    window::{clear_background, next_frame},
};

use crate::{
    math::math::Vec2f,
    physics::physics_engine::PhysicsEngine,
    renderer::{component::Position, entity::EntityManager, ui::UiManager},
};

pub struct AppContext {
    pub entity_manager: EntityManager,
    pub ui_manager: UiManager,
}

impl AppContext {
    pub fn get_window_bounds() -> (f32, f32) {
        screen_size()
    }

    pub fn get_button_press(mouse_button: MouseButton) -> bool {
        is_mouse_button_down(mouse_button)
    }

    pub fn get_mouse_position() -> Vec2f {
        let pos = mouse_position();
        Vec2f { x: pos.0, y: pos.1 }
    }
}

pub struct WindowParameters {
    pub width: u32,
    pub height: u32,
}

pub struct App<S> {
    pub app_context: AppContext,
    pub physics_engine: PhysicsEngine,
    pub systems: Vec<fn(&mut AppContext, f32, &mut S)>,
    pub state: S,
}

impl<S> App<S> {
    pub fn new(state: S, window_params: WindowParameters) -> Self {
        set_window_size(window_params.width, window_params.height);

        Self {
            app_context: AppContext {
                entity_manager: EntityManager::init(),
                ui_manager: UiManager::init(),
            },
            physics_engine: PhysicsEngine::init(10.),
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
            self.physics_engine
                .update(&mut self.app_context.entity_manager);
            self.app_context.entity_manager.render_all();
            next_frame().await;
        }
    }
}
