use std::collections::HashMap;

use macroquad::time::get_frame_time;

use crate::{
    app::{self, AppContext},
    math::math::Vec2f,
    physics::entities::physics_body::RigidBody,
    renderer::entity::{Entity, EntityId},
};

pub struct PhysicsEngine {}

const GRAVITY_CONST: f32 = 9.81;
const GRAVITY_DIR: Vec2f = Vec2f { x: 0., y: -1. };

impl PhysicsEngine {
    pub fn init() -> Self {
        Self {}
    }

    // Sort and sweep broad phase
    fn broad(entities: &HashMap<EntityId, Entity>) -> Vec<(EntityId, EntityId)> {
        let mut sorted = entities
            .iter()
            .map(|(id, e)| {
                (
                    e.bounding_box.x - e.bounding_box.w / 2.,
                    e.bounding_box.x + e.bounding_box.w / 2.,
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

    fn narrow(
        pairs: &Vec<(EntityId, EntityId)>,
        entities: &HashMap<EntityId, Entity>,
    ) -> Vec<(EntityId, EntityId)> {
        pairs
            .iter()
            .filter(|(a, b)| {
                let a = entities.get(&a).unwrap();
                let b = entities.get(&b).unwrap();

                b.bounding_box.intersects(&a.bounding_box)
            })
            .cloned()
            .collect()
    }

    fn handle_collision(a: &EntityId, b: &EntityId, entities: &mut HashMap<EntityId, Entity>) {
        let ae = entities.get(&a).unwrap();
        let be = entities.get(&b).unwrap();

        let v_a = ae.physics_body.velocity;
        let v_b = be.physics_body.velocity;

        let s_a = ae.physics_body.position;
        let s_b = be.physics_body.position;

        let a_invm = ae.physics_body.inv_mass;
        let b_invm = be.physics_body.inv_mass;

        let m_a = ae.physics_body.mass;
        let m_b = be.physics_body.mass;
        // J = F*dt = ma*dt = m/(v*dt)*dt = m*dv => dv = J/m
        let cor = ae.physics_body.cor.min(be.physics_body.cor);
        let v_rel = v_a - v_b;
        let mut n = (s_b - s_a).norm();
        if n.dot(&v_rel) > 0.0 {
            n = -n; // Ensure normal opposes relative velocity
        }

        // How much of the relative velocity is in the direction of n
        let j = -(1. + cor) * v_rel.dot(&n) / (a_invm + b_invm);
        if a_invm > 0.0 && m_a > 0. {
            let v_an = v_a + a_invm * j * n;
            entities.get_mut(&a).unwrap().physics_body.velocity = v_an;
        }
        if b_invm > 0.0 && m_b > 0. {
            let v_bn = v_b - b_invm * j * n;
            entities.get_mut(&b).unwrap().physics_body.velocity = v_bn;
        }
    }
    pub fn update(&self, app_context: &mut AppContext) {
        let dt = get_frame_time();

        // 1. FIRST: Integrate forces and update positions
        for (_, entity) in &mut app_context.entity_manager.entities {
            if entity.rigidbody == RigidBody::Static || entity.physics_body.mass <= 0. {
                continue;
            }

            // G = mg = ma
            let G = GRAVITY_DIR * GRAVITY_CONST * entity.physics_body.mass;
            entity.physics_body.force_accumulator += G;

            let (inv_mass, forces) = (
                entity.physics_body.inv_mass,
                entity.physics_body.force_accumulator,
            );

            let p_body = &mut entity.physics_body;

            let a = forces * inv_mass;
            p_body.velocity += a * dt;
            p_body.position += p_body.velocity * dt;
            entity.bounding_box.x = p_body.position.x;
            entity.bounding_box.y = p_body.position.y;

            p_body.force_accumulator = Vec2f::zero();
        }

        // 2. THEN: Detect and resolve collisions
        let possible_collision_pairs = Self::broad(&app_context.entity_manager.entities);
        let collision_pairs = Self::narrow(
            &possible_collision_pairs,
            &app_context.entity_manager.entities, // Note: not &mut
        );

        for pair in &collision_pairs {
            Self::handle_collision(&pair.0, &pair.1, &mut app_context.entity_manager.entities);
        }
    }
}
