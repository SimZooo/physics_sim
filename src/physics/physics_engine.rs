use macroquad::{miniquad::window::screen_size, time::get_frame_time};

use crate::{math::math::Vec2f, renderer::entity::EntityManager};

pub struct PhysicsEngine {
    scale: f32,
}

const GRAVITY_CONST: f32 = 9.81;
const GRAVITY_DIR: Vec2f = Vec2f { x: 0., y: 1. };

impl PhysicsEngine {
    pub fn init(scale: f32) -> Self {
        Self { scale }
    }

    pub fn update(&self, manager: &mut EntityManager) {
        let dt = get_frame_time();
        let window_size = screen_size();
        for (_, entity) in &mut manager.entities {
            if let Some(body) = &mut entity.physics_body {
                // G = mg = ma
                let a = GRAVITY_DIR * GRAVITY_CONST * self.scale;
                let v = body.velocity + a * dt;
                let next_pos = body.position + v * dt;
                if next_pos.y >= window_size.1 {
                    let v = Vec2f { x: v.x, y: -v.y };
                    let next_pos = body.position + v * dt;
                    body.position = next_pos;
                    body.velocity = v;
                } else {
                    body.position = next_pos;
                    body.velocity = v;
                }
                body.acceleration = a;
            }
        }
    }
}
