use std::ops;
use fixed::types::I16F16;

// Float: 84.13

pub type Float = I16F16;

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x : Float,
    pub y : Float,
    pub z : Float,
}

impl Vec3 {
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

    pub fn new(a: Float, b: Float, c: Float) -> Vec3 {
        Vec3 {
            x: a,
            y: b,
            z: c,
        }
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl_op_ex_commutative!(* |a: &Vec3, b: &Float| -> Vec3 {
    Vec3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});

impl_op_ex!(/ |a: &Vec3, b: &Float| -> Vec3 {
    Vec3 {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
    }
});

impl_op_ex!(* |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
    }
});

impl_op_ex!(/ |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x / b.x,
        y: a.y / b.y,
        z: a.z / b.z,
    }
});

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
});

impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});

impl_op_ex!(% |a: &Vec3, b: &Vec3| -> Float {
    a.x * b.x + a.y * b.y + a.z * b.z
});

impl_op!(*= |a: &mut Vec3, b: &Vec3| {
    *a = &(*a) * b;
});

impl_op!(*= |a: &mut Vec3, b: Vec3| {
    *a = &(*a) * b;
});

impl_op!(+= |a: &mut Vec3, b: Vec3| {
    *a = &(*a) + b;
});

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at(&self, x: Float) -> Vec3 {
        &self.origin + (&self.direction * x)
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
