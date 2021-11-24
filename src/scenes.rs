use rand::random;

use crate::material::Material::*;
use crate::{Cylinder, Float, HitableGroup, KDTree, Plane, Sphere, Vec3};

#[allow(dead_code)]
pub fn pipedream() -> HitableGroup {
    let mut planes: Vec<Plane> = Vec::new();
    let mut cylinders: Vec<Cylinder> = Vec::new();

    planes.push(Plane {
        origin: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        material: Lambertian(Vec3::new(0.2, 0.2, 0.2)),
    });

    planes.push(Plane {
        origin: Vec3::new(60.0 * 60.0 * 2.0, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        material: Lambertian(Vec3::new(0.2, 0.2, 0.2)),
    });

    cylinders.push(Cylinder {
        radius: 10.0,
        material: Sun,
    });

    cylinders.push(Cylinder {
        radius: 60.0 * 8.0,
        material: BiasedLambertian(Vec3::new(0.2, 0.7, 0.2)),
    });

    HitableGroup {
        planes,
        spheres: KDTree::new(Vec::new()),
        cylinders,
    }
}

#[allow(dead_code)]
pub fn plain_scene() -> HitableGroup {
    let mut things: Vec<Sphere> = Vec::new();
    let mut planes: Vec<Plane> = Vec::new();

    planes.push(Plane {
        origin: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        material: Metal(Vec3::new(0.5, 0.6, 0.75), 0.9),
    });

    things.push(Sphere {
        center: Vec3::new(-2.0, 0.0, 0.0),
        radius: 0.5,
        material: Metal(Vec3::new(0.1, 0.1, 0.1), 0.5),
    });

    HitableGroup {
        spheres: KDTree::new(things),
        planes,
        cylinders: Vec::new(),
    }
}

pub fn random_scene() -> HitableGroup {
    let mut things: Vec<Sphere> = Vec::new();
    let mut planes: Vec<Plane> = Vec::new();

    planes.push(Plane {
        origin: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        material: Lambertian(Vec3::new(0.5, 0.5, 0.5)),
    });

    things.push(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric(Vec3::new(1.0, 1.0, 1.0), 1.5),
    });

    things.push(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian(Vec3::new(0.4, 0.2, 0.1)),
    });

    things.push(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = random();
            let center = Vec3::new(
                a as f32 + random::<f32>() * 0.9,
                0.2,
                b as f32 + random::<f32>() * 0.9,
            );
            let offset = Vec3::new(4.0, 0.2, 0.0);

            if (&center - offset).norm() > 0.9 {
                things.push(Sphere {
                    center,
                    radius: 0.2,
                    material: if choose_mat < 0.6 {
                        Lambertian(Vec3::random() * Vec3::random())
                    } else if choose_mat < 0.95 {
                        Metal(
                            Vec3::random() * 0.5 + Vec3::new(0.5, 0.5, 0.5),
                            random::<f32>() * 0.5,
                        )
                    } else {
                        Dielectric(Vec3::new(1.0, 1.0, 1.0), 1.5)
                    },
                });
            }
        }
    }

    let cylinders = Vec::new();

    HitableGroup {
        spheres: KDTree::new(things),
        planes,
        cylinders,
    }
}
