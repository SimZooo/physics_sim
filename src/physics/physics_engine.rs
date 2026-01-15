use std::collections::HashMap;

use macroquad::{miniquad::window::screen_size, time::get_frame_time};

use crate::{
    app::AppContext,
    math::math::Vec2f,
    physics::entities::physics_body::RigidBody,
    renderer::entity::{Entity, EntityId, EntityManager},
};

pub struct PhysicsEngine {}

const GRAVITY_CONST: f32 = 9.81;
const GRAVITY_DIR: Vec2f = Vec2f { x: 0., y: -1. };

impl PhysicsEngine {
    pub fn init() -> Self {
        Self {}
    }

    // Sort and sweep broad phase
    pub fn broad(
        &self,
        entities: &HashMap<EntityId, Entity>,
        ppu: f32,
    ) -> Vec<(EntityId, EntityId)> {
        let mut sorted = entities
            .iter()
            .map(|(id, e)| {
                (
                    e.bounding_box.x,
                    e.bounding_box.x + e.bounding_box.w / ppu,
                    id,
                )
            })
            .collect::<Vec<(f32, f32, &EntityId)>>();
        sorted.sort_by(|a, b| a.0.total_cmp(&b.0));

        let mut potential_collisions: Vec<(EntityId, EntityId)> = vec![];
        let mut active_collisions: Vec<(f32, EntityId)> = vec![];

        for (min_x_a, max_x_a, id_a) in sorted.iter() {
            active_collisions.retain(|(other_max_x, _)| *other_max_x >= *min_x_a);

            for &(_, other_id) in &active_collisions {
                potential_collisions.push((other_id, **id_a))
            }

            active_collisions.push((*max_x_a, **id_a));
        }

        potential_collisions
    }

    pub fn update(&self, app_context: &mut AppContext) {
        let collision_pairs = self.broad(&app_context.entity_manager.entities, app_context.ppu);
        println!("{:?}", collision_pairs);

        let dt = get_frame_time();
        for (_, entity) in &mut app_context.entity_manager.entities {
            if entity.rigidbody == RigidBody::Static {
                continue;
            }

            // G = mg = ma
            let a = GRAVITY_DIR * GRAVITY_CONST;
            let v = entity.physics_body.velocity + a * dt;
            let next_pos = entity.physics_body.position + v * dt;
            if next_pos.y <= 0. {
                let v = Vec2f { x: v.x, y: -v.y };
                let next_pos = entity.physics_body.position + v * dt;
                entity.physics_body.position = next_pos;
                entity.physics_body.velocity = v;
            } else {
                entity.physics_body.position = next_pos;
                entity.physics_body.velocity = v;
            }
            entity.physics_body.acceleration = a;
        }
    }
}
