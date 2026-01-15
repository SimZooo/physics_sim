pub struct Vec2u {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0., y: 0. }
    }
}
impl std::ops::Mul<f32> for Vec2f {
    type Output = Vec2f;
    fn mul(self, rhs: f32) -> Self {
        Vec2f {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::MulAssign<f32> for Vec2f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl std::ops::AddAssign<Vec2f> for Vec2f {
    fn add_assign(&mut self, rhs: Vec2f) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl std::ops::Add<Vec2f> for Vec2f {
    type Output = Vec2f;
    fn add(self, rhs: Vec2f) -> Vec2f {
        Vec2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Neg for Vec2f {
    type Output = Vec2f;
    fn neg(self) -> Vec2f {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}
