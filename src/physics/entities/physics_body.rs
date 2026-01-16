use macroquad::{
    color::BLUE,
    shapes::draw_line,
    window::{screen_height, screen_width},
};

use crate::math::math::{Vec2f, Vec2i};

#[derive(PartialEq, Eq)]
pub enum RigidBody {
    Dynamic,
    Static,
}

#[derive(Debug, Clone, Copy)]
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

    pub fn intersects(&self, b: &BoundingBox) -> bool {
        // self.x and self.y are CENTER coordinates
        let a_left = self.x - self.w / 2.0;
        let a_right = self.x + self.w / 2.0;
        let a_top = self.y - self.h / 2.0;
        let a_bottom = self.y + self.h / 2.0;

        let b_left = b.x - b.w / 2.0;
        let b_right = b.x + b.w / 2.0;
        let b_top = b.y - b.h / 2.0;
        let b_bottom = b.y + b.h / 2.0;

        a_left < b_right && a_right > b_left && a_top < b_bottom && a_bottom > b_top
    }

    // Helper method to get edges
    pub fn left(&self) -> f32 {
        self.x - self.w / 2.0
    }
    pub fn right(&self) -> f32 {
        self.x + self.w / 2.0
    }
    pub fn top(&self) -> f32 {
        self.y - self.h / 2.0
    }
    pub fn bottom(&self) -> f32 {
        self.y + self.h / 2.0
    }
}

pub struct Force {
    pub direction: Vec2i,
    pub magnitude: f32,
}

pub struct PhysicsBody {
    pub position: Vec2f,
    pub velocity: Vec2f,
    pub angular_velocity: f32,
    pub acceleration: Vec2f,
    pub mass: f32,
    pub inv_mass: f32,
    pub cor: f32,
    pub force_accumulator: Vec2f,
}

impl PhysicsBody {
    pub fn new(position: Vec2f, mass: f32, cor: f32) -> Self {
        PhysicsBody {
            position,
            velocity: Vec2f::zero(),
            angular_velocity: 0.,
            acceleration: Vec2f::zero(),
            mass,
            inv_mass: if mass <= 0. { 0. } else { 1. / mass },
            cor,
            force_accumulator: Vec2f::zero(),
        }
    }
}
