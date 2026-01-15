use egui_macroquad::egui::{Pos2, Rect};
use macroquad::{
    color::{Color, RED},
    input::{MouseButton, is_mouse_button_down, mouse_position},
    miniquad::window::{screen_size, set_window_size},
    window::{clear_background, next_frame, screen_height},
};

use crate::{
    math::math::Vec2f,
    physics::{
        entities::physics_body::{PhysicsBody, RigidBody},
        physics_engine::PhysicsEngine,
    },
    renderer::{
        entity::{Entity, EntityManager, Shape},
        ui::UiManager,
    },
};

pub struct AppContext {
    pub entity_manager: EntityManager,
    pub ui_wants_pointer: bool,
    pub ui_wants_keyboard: bool,
    pub ppu: f32,
    pub side_panel_left_rect: Rect,
    pub debug_outlines: bool,
}

impl AppContext {
    pub fn get_window_bounds() -> (f32, f32) {
        screen_size()
    }

    pub fn get_button_press(&self, mouse_button: MouseButton) -> bool {
        if self.ui_wants_pointer
            || self
                .side_panel_left_rect
                .contains(Pos2::from(mouse_position()))
        {
            return false;
        }
        is_mouse_button_down(mouse_button)
    }

    pub fn get_mouse_position(&self) -> Vec2f {
        let pos = mouse_position();
        Vec2f {
            x: pos.0 / self.ppu,
            y: (screen_height() - pos.1) / self.ppu,
        }
    }

    pub fn new_entity(
        &mut self,
        position: Vec2f,
        mass: f32,
        size: Vec2f,
        color: Color,
        shape: Shape,
        rigidbody: RigidBody,
    ) {
        let physics_body = PhysicsBody::new(position, mass);
        let ent = Entity::new(size, color, physics_body, shape, rigidbody);
        self.entity_manager.add(ent);
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
    pub paused: bool,
    pub state: S,
}

impl<S> App<S> {
    pub fn new(state: S, window_params: WindowParameters) -> Self {
        set_window_size(window_params.width, window_params.height);

        Self {
            app_context: AppContext {
                entity_manager: EntityManager::init(),
                ui_wants_keyboard: false,
                ui_wants_pointer: false,
                side_panel_left_rect: Rect {
                    min: Pos2::new(0., 0.),
                    max: Pos2::new(0., 0.),
                },
                ppu: 100.,
                debug_outlines: false,
            },
            physics_engine: PhysicsEngine::init(),
            systems: vec![],
            paused: false,
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

            UiManager::render_ui(self);

            clear_background(Color::from_hex(0x252526));
            if !self.paused {
                self.physics_engine.update(&mut self.app_context);
            }
            self.app_context
                .entity_manager
                .render_all(self.app_context.ppu, self.app_context.debug_outlines);

            egui_macroquad::draw();
            next_frame().await;
        }
    }
}
