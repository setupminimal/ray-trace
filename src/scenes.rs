use rand::random;

use crate::material::Material::*;
use crate::shapes::*;
use crate::{Color, Scene, Vec3};

pub fn one_sphere() -> Scene {
    let mut s = Scene::empty();

    s.add(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        mat: Metal(Color::new(0.75, 0.75, 0.75), 0.001),
    });

    s
}

pub fn two_spheres() -> Scene {
    let mut scene = one_sphere();
    scene.add(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat: DiffuseCos(Color::new(0.5, 0.5, 0.5)),
    });
    scene
}

pub fn flanking_mirrors() -> Scene {
    let mut scene = Scene::empty();

    let ground = DiffuseCos(Color::new(0.8, 0.8, 0.0));
    let center = Dielectric(Color::new(1.0, 1.0, 1.0), 1.5);
    //DiffuseCos(Color::new(0.7, 0.3, 0.3));
    let left = Metal(Color::new(0.8, 0.8, 0.8), 0.1);
    let right = Metal(Color::new(0.8, 0.6, 0.2), 0.5);

    scene.add(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat: ground,
    });

    scene.add(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        mat: center,
    });

    scene.add(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat: left,
    });

    scene.add(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        mat: right,
    });

    scene
}

pub fn random_scene() -> Scene {
    let mut scene = Scene::empty();

    scene.add(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: DiffuseCos(Color::new(0.5, 0.5, 0.5)),
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f32>();
            let center = Vec3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );

            scene.add(Sphere {
                center,
                radius: 0.2,
                mat: if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    DiffuseCos(albedo)
                } else if choose_mat < 0.95 {
                    let albedo = Color::random() * 0.5 + Color::new(0.5, 0.5, 0.5);
                    Metal(albedo, random::<f32>() * 0.5)
                } else {
                    Dielectric(Color::new(1.0, 1.0, 1.0), 1.5)
                },
            });
        }
    }

    scene.add(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: Dielectric(Vec3::new(1.0, 1.0, 1.0), 1.5),
    });

    scene.add(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: DiffuseCos(Vec3::new(0.4, 0.2, 0.1)),
    });

    scene.add(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
    });

    scene
}
