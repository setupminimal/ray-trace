use crate::vec::*;

pub fn sample_sphere() -> Vec3 {
    let mut v = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    while v.norm() > 1.0 {
        v = Vec3 {
            x: random(),
            y: random(),
            z: random(),
        }
    }

    v
}

pub fn sample_disk() -> Vec3 {
    let diff = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 0.0,
    };
    let mut p = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    while &p % &p >= 1.0 {
        p = Vec3 {
            x: random(),
            y: random(),
            z: 0.0,
        };

        p = p * 2.0 - &diff;
    }

    p
}

pub fn random() -> Float {
    rand::random::<Float>()
}
