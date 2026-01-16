use egui_macroquad::egui::{Pos2, Rect};
use macroquad::{
    color::{Color, RED},
    input::{MouseButton, is_mouse_button_down, mouse_position, mouse_position_local},
    miniquad::window::{screen_size, set_window_size},
    window::{clear_background, next_frame, screen_height, screen_width},
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
    pub show_forces: bool,
    pub show_com: bool,
    pub physics_dimensions: Vec2f,
    pub current_shape: Shape,
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
        let (mx, my) = mouse_position();

        Vec2f {
            x: mx / screen_width() * self.physics_dimensions.x,
            y: (screen_height() - my) / screen_height() * self.physics_dimensions.y,
        }
    }

    pub fn new_entity(
        &mut self,
        position: Vec2f,
        mass: f32,
        size: Vec2f,
        color: Color,
        rigidbody: RigidBody,
    ) {
        let physics_body = PhysicsBody::new(position, mass, 0.3);
        let ent = Entity::new(
            size,
            color,
            physics_body,
            self.current_shape.clone(),
            rigidbody,
        );
        self.entity_manager.add(ent);
    }

    pub fn new_entity_shaped(
        &mut self,
        position: Vec2f,
        mass: f32,
        size: Vec2f,
        color: Color,
        shape: Shape,
        rigidbody: RigidBody,
    ) {
        let physics_body = PhysicsBody::new(position, mass, 0.3);
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
    pub fn new(state: S, window_params: WindowParameters, physics_dimensions: Vec2f) -> Self {
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
                physics_dimensions,
                current_shape: Shape::Circle,
                show_forces: false,
                show_com: false,
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
            self.app_context.entity_manager.render_all(
                self.app_context.physics_dimensions,
                self.app_context.debug_outlines,
                self.app_context.show_forces,
                self.app_context.show_com,
            );

            egui_macroquad::draw();
            next_frame().await;
        }
    }
}
