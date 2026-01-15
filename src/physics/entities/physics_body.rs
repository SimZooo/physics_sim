use crate::math::math::{Vec2f, Vec2i};

pub struct Force {
    pub direction: Vec2i,
    pub magnitude: f32,
}

pub struct PhysicsBody {
    pub position: Vec2f,
    pub velocity: Vec2f,
    pub acceleration: Vec2f,
    pub mass: f32,
    pub inv_mass: f32,

    force_accumulator: Vec2f,
}

impl PhysicsBody {
    pub fn new(position: Vec2f, mass: f32) -> Self {
        PhysicsBody {
            position,
            velocity: Vec2f::zero(),
            acceleration: Vec2f::zero(),
            mass,
            inv_mass: 1. / mass,
            force_accumulator: Vec2f::zero(),
        }
    }
}
