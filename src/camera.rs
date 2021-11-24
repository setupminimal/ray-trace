use crate::sample::*;
use crate::shape::*;
use crate::vec::*;

use std::f32::consts::PI;

fn tfog(ray: Ray, t: Float) -> Float {
    ((ray.origin.x + ray.direction.x * t).sin() * (ray.origin.z + ray.direction.z * t).sin()
        / (ray.origin.y + ray.direction.y * t))
        .powi(2)
}

fn fog(point: Vec3) -> Float {
    (point.x.sin() * point.z.sin() / point.y).powi(2)
}

fn dfog(point: Vec3, dir: Vec3) -> Float {
    let ev = point.x.sin() * point.z.sin() / point.y;
    let grad = Vec3::new(
        2.0 * ev * point.x.cos(),
        -2.0 * ev / point.y.powi(2),
        2.0 * ev * point.z.cos(),
    );

    grad % dir
}

fn color(ray: Ray, scene: &HitableGroup) -> Vec3 {
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    let black = Vec3::new(0.0, 0.0, 0.0);

    let mut color = Vec3::new(1.0, 1.0, 1.0);
    let mut current_ray = ray;
    let mut iterations = 0;

    while iterations < 100 && color.norm() > 0.02 {
        let hit = scene.hit(&current_ray, 0.001, 1_000_000.0);
        match hit {
            None => {
                let direction = current_ray.direction.to_unit();
                let t = 0.5 * (direction.y + 1.0);
                color *= (white * (1.0 - t)) + (blue * t);
                break;
            }
            Some(hit) => {
                let scatter = hit.material.scatter(&current_ray, hit.point, hit.normal);
                color *= scatter.attenuation;
                iterations += 1;
                match scatter.ray {
                    Some(r) => {
                        current_ray = r;
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    }

    color
}

pub struct Camera {
    samples: u16,
    lens_radius: Float,
    u: Vec3,
    v: Vec3,
    nx: Float,
    ny: Float,
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        nx: u16,
        ny: u16,
        vfov: Float,
        aperture: Float,
        samples: u16,
    ) -> Camera {
        let focus_dist = (&lookfrom - &lookat).norm();

        let w = (&lookfrom - &lookat).to_unit();
        let u = vup.cross(&w).to_unit();
        let v = w.cross(&u);

        let aspect = nx as Float / ny as Float;
        let theta = vfov * PI * (1.0 / 180.0);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        Camera {
            samples,
            lens_radius: aperture / 2.0,
            nx: nx as Float,
            ny: ny as Float,
            lower_left: &lookfrom
                - (&u * half_width * focus_dist)
                - (&v * half_height * focus_dist)
                - (&w * focus_dist),
            horizontal: 2.0 * half_width * focus_dist * &u,
            vertical: 2.0 * half_height * focus_dist * &v,
            origin: lookfrom,
            u,
            v,
        }
    }

    pub fn point(&self, x: u16, y: u16, world: &HitableGroup) -> Vec3 {
        let mut acc_color = Vec3::new(0.0, 0.0, 0.0);
        for _ in 1..self.samples {
            let rd = sample_disk() * self.lens_radius;
            let offset = &self.u * rd.x + &self.v * rd.y;
            let u = (x as Float + random()) / self.nx;
            let v = (y as Float + random()) / self.ny;
            let ray = Ray::new(
                &self.origin + &offset,
                &self.lower_left
                    + (u * &self.horizontal)
                    + (v * &self.vertical)
                    + self.origin.negate()
                    + offset.negate(),
            );

            let val = color(ray, world);
            acc_color += val;
        }

        acc_color / (self.samples as Float)
    }
}

// TODO add tests:
// - Image render with no light sources (incl backround light)
//   should be completely dark
