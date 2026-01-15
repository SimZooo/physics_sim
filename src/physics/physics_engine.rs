use macroquad::{miniquad::window::screen_size, time::get_frame_time};

use crate::{math::math::Vec2f, renderer::entity::EntityManager};

pub struct PhysicsEngine {}

const GRAVITY_CONST: f32 = 9.81;
const GRAVITY_DIR: Vec2f = Vec2f { x: 0., y: -1. };

impl PhysicsEngine {
    pub fn init() -> Self {
        Self {}
    }

    pub fn update(&self, manager: &mut EntityManager) {
        let dt = get_frame_time();
        for (_, entity) in &mut manager.entities {
            if let Some(body) = &mut entity.physics_body {
                // G = mg = ma
                let a = GRAVITY_DIR * GRAVITY_CONST;
                let v = body.velocity + a * dt;
                let next_pos = body.position + v * dt;
                if next_pos.y <= 0. {
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
