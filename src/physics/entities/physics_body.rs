use macroquad::window::{screen_height, screen_width};

use crate::math::math::{Vec2f, Vec2i};

#[derive(PartialEq, Eq)]
pub enum RigidBody {
    Dynamic,
    Static,
}

pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl BoundingBox {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn intersects(&self, b: &BoundingBox, ppu: f32) -> bool {
        self.x < b.x + b.w / ppu
            && self.x + self.w / ppu > b.x
            && self.y < b.y + b.h / ppu
            && self.y + self.h / ppu > b.y
    }
}

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
