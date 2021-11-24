use std::ops;
use Direction::*;

pub type Float = f32;

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

#[derive(Copy, Clone)]
pub enum Direction {
    X,
    Y,
    Z,
}

impl Vec3 {
    pub const fn cardinal_directions() -> [Vec3; 3] {
        [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ]
    }

    pub fn elem_min(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            if self.x < other.x { self.x } else { other.x },
            if self.y < other.y { self.y } else { other.y },
            if self.z < other.z { self.z } else { other.z },
        )
    }

    pub fn elem_max(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            if self.x > other.x { self.x } else { other.x },
            if self.y > other.y { self.y } else { other.y },
            if self.z > other.z { self.z } else { other.z },
        )
    }

    pub fn get(&self, dir: Direction) -> Float {
        match dir {
            X => self.x,
            Y => self.y,
            Z => self.z,
        }
    }

    pub fn longest_dimension(&self) -> Direction {
        if self.x > self.y {
            if self.x > self.z {
                X
            } else {
                Z
            }
        } else if self.y > self.z {
            Y
        } else {
            Z
        }
    }

    pub fn norm(&self) -> Float {
        (self % self).sqrt()
    }

    pub fn to_unit(&self) -> Vec3 {
        let n = self.norm();
        Vec3 {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }

    pub fn as_ppm(&self) -> String {
        let r = (self.x.sqrt() * 255.99).floor();
        let g = (self.y.sqrt() * 255.99).floor();
        let b = (self.y.sqrt() * 255.99).floor();
        format!("{} {} {}\n", r, g, b)
    }

    pub fn negate(&self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub const fn new(a: Float, b: Float, c: Float) -> Vec3 {
        Vec3 { x: a, y: b, z: c }
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<'a, 'b> ops::Mul<&'a Float> for &'b Vec3 {
    type Output = Vec3;

    fn mul(self, b: &Float) -> Self::Output {
        let a = self;
        Vec3 {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
        }
    }
}

impl<'a> ops::Mul<Float> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, b: Float) -> Self::Output {
        let a = self;
        Vec3 {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
        }
    }
}

impl<'a> ops::Mul<&'a Float> for Vec3 {
    type Output = Vec3;

    fn mul(self, b: &Float) -> Self::Output {
        let a = self;
        Vec3 {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
        }
    }
}

impl ops::Mul<Float> for Vec3 {
    type Output = Vec3;

    fn mul(self, b: Float) -> Self::Output {
        let a = self;
        Vec3 {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
        }
    }
}

impl<'a, 'b> ops::Mul<&'a Vec3> for &'b Float {
    type Output = Vec3;

    fn mul(self, a: &Vec3) -> Self::Output {
        let b = self;
        Vec3 {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
        }
    }
}
impl<'a> ops::Mul<Vec3> for &'a Float {
    type Output = Vec3;

    fn mul(self, a: Vec3) -> Self::Output {
        let b = self;
        Vec3 {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
        }
    }
}

impl<'a> ops::Mul<&'a Vec3> for Float {
    type Output = Vec3;

    fn mul(self, a: &Vec3) -> Self::Output {
        let b = self;
        Vec3 {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
        }
    }
}

impl ops::Mul<Vec3> for Float {
    type Output = Vec3;

    fn mul(self, a: Vec3) -> Self::Output {
        let b = self;
        Vec3 {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
        }
    }
}

impl<'a, 'b> ops::Div<&'a Float> for &'b Vec3 {
    type Output = Vec3;

    fn div(self, b: &Float) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x / b,
            y: a.y / b,
            z: a.z / b,
        }
    }
}

impl<'a> ops::Div<Float> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, b: Float) -> Self::Output {
        let a = self;
        Vec3 {
            x: a.x / b,
            y: a.y / b,
            z: a.z / b,
        }
    }
}

impl<'a> ops::Div<&'a Float> for Vec3 {
    type Output = Vec3;

    fn div(self, b: &Float) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x / b,
            y: a.y / b,
            z: a.z / b,
        }
    }
}

impl ops::Div<Float> for Vec3 {
    type Output = Vec3;

    fn div(self, b: Float) -> Self::Output {
        let a = self;
        Vec3 {
            x: a.x / b,
            y: a.y / b,
            z: a.z / b,
        }
    }
}

impl ops::Div<&Vec3> for Float {
    type Output = Vec3;

    fn div(self, a: &Vec3) -> Self::Output {
        let b = self;
        Vec3 {
            x: a.x / b,
            y: a.y / b,
            z: a.z / b,
        }
    }
}

impl<'a, 'b> ops::Mul<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;

    fn mul(self, b: &Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
        }
    }
}

impl<'a> ops::Mul<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, b: Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
        }
    }
}

impl<'a> ops::Mul<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, b: &Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, b: Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
        }
    }
}

impl<'a, 'b> ops::Div<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;

    fn div(self, b: &Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x / b.x,
            y: a.y / b.y,
            z: a.z / b.z,
        }
    }
}

impl<'a> ops::Div<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, b: Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x / b.x,
            y: a.y / b.y,
            z: a.z / b.z,
        }
    }
}

impl<'a> ops::Div<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, b: &Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x / b.x,
            y: a.y / b.y,
            z: a.z / b.z,
        }
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, b: Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x / b.x,
            y: a.y / b.y,
            z: a.z / b.z,
        }
    }
}

impl<'a, 'b> ops::Add<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;

    fn add(self, b: &Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    }
}

impl<'a> ops::Add<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, b: Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    }
}

impl<'a> ops::Add<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, b: &Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, b: Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    }
}

impl<'a, 'b> ops::Sub<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;

    fn sub(self, b: &Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
        }
    }
}

impl<'a> ops::Sub<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, b: Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
        }
    }
}

impl<'a> ops::Sub<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, b: &Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, b: Vec3) -> Self::Output {
        let a = self;

        Vec3 {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
        }
    }
}

impl<'a, 'b> ops::Rem<&'a Vec3> for &'b Vec3 {
    type Output = Float;

    fn rem(self, b: &Vec3) -> Self::Output {
        let a = self;
        a.x * b.x + a.y * b.y + a.z * b.z
    }
}

impl<'a> ops::Rem<Vec3> for &'a Vec3 {
    type Output = Float;

    fn rem(self, b: Vec3) -> Self::Output {
        let a = self;

        a.x * b.x + a.y * b.y + a.z * b.z
    }
}

impl<'a> ops::Rem<&'a Vec3> for Vec3 {
    type Output = Float;

    fn rem(self, b: &Vec3) -> Self::Output {
        let a = self;

        a.x * b.x + a.y * b.y + a.z * b.z
    }
}

impl ops::Rem<Vec3> for Vec3 {
    type Output = Float;

    fn rem(self, b: Vec3) -> Self::Output {
        let a = self;

        a.x * b.x + a.y * b.y + a.z * b.z
    }
}

impl<'a> ops::MulAssign<&'a Vec3> for Vec3 {
    fn mul_assign(&mut self, b: &Vec3) {
        *self = &(*self) * b;
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, b: Vec3) {
        *self = &(*self) * b;
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, b: Vec3) {
        *self = &(*self) + b;
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub dir_inv: Vec3,
}

impl Ray {
    pub fn point_at(&self, x: Float) -> Vec3 {
        &self.origin + (&self.direction * x)
    }

    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            dir_inv: 1.0 / &direction,
            direction,
        }
    }
}

pub fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    v - 2.0 * (v % normal) * normal
}

pub fn refract(v: &Vec3, normal: &Vec3, nint: Float) -> Option<Vec3> {
    let uv = v.to_unit();
    let dt = &uv % normal;
    let discriminant = 1.0 - nint * nint * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((nint * (uv - normal * dt)) - (normal * discriminant.powf(0.5)))
    } else {
        None
    }
}

pub fn schlick(cosine: Float, ref_idx: Float) -> Float {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
