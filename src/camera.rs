use rand::random;

use crate::{Ray, Vec3};

pub struct Camera {
    width: f32,
    height: f32,
    focal_length: f32,
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(vfov: f32, aspect_ratio: f32, lookfrom: Vec3, lookat: Vec3) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let height = 2.0 * h;
        let width = height * aspect_ratio;

        let w = (&lookfrom - &lookat).to_unit();
        let u = Vec3::new(0.0, 1.0, 0.0).cross(&w).to_unit();
        let v = w.cross(&u);

        let horizontal = width * u;
        let vertical = height * v;
        let lower_left_corner = &lookfrom - &horizontal / 2.0 - &vertical / 2.0 - w;

        let origin = lookfrom;

        Camera {
            width,
            height,
            focal_length: 10.0,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn ray(&self, x: usize, y: usize, width: usize, height: usize) -> Vec<Ray> {
        let mut rays = Vec::new();
        // TODO configurable samples
        for _ in 0..500 {
            let u: f32 = (x as f32 + random::<f32>()) / width as f32;
            let v: f32 = (y as f32 + random::<f32>()) / height as f32;
            rays.push(Ray::new(
                // This clone is because rays will be created
                // dynamically, and so need to own their origin
                // vectors.
                self.origin.clone(),
                &self.lower_left_corner + u * &self.horizontal + v * &self.vertical - &self.origin,
            ))
        }

        rays
    }
}
