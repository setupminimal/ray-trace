use rand::random;

use crate::vec::*;

use Material::*;

#[derive(Clone)]
pub enum Material {
    DiffuseCos3(Vec3),
    DiffuseCos(Vec3), // 'Best'
    UniformDiffuse(Vec3),
    Metal(Vec3, Float),
    Dielectric(Vec3, Float),
    BiasedLambertian(Vec3), // Nonphysical?
}

pub struct ScatterRecord<'a> {
    pub attenuation: &'a Vec3,
    pub ray: Option<Ray>,
}

impl Material {
    pub fn scatter(&self, ray: &Ray, point: Vec3, normal: Vec3, front_face: bool) -> ScatterRecord {
        match self {
            DiffuseCos3(albedo) => {
                let target = normal + Vec3::random_in_unit_sphere();
                ScatterRecord {
                    attenuation: albedo,
                    ray: Some(Ray::new(point, target)),
                }
            }
            DiffuseCos(albedo) => {
                let target = &normal + Vec3::random_unit_vector();
                ScatterRecord {
                    attenuation: albedo,
                    ray: Some(Ray::new(point, target)),
                }
            }
            UniformDiffuse(albedo) => {
                let target = Vec3::random_in_hemisphere(normal);
                ScatterRecord {
                    attenuation: albedo,
                    ray: Some(Ray::new(point, target)),
                }
            }
            BiasedLambertian(albedo) => {
                let target = normal + Vec3::random_unit_vector() * 0.10;
                ScatterRecord {
                    attenuation: albedo,
                    ray: Some(Ray::new(point, target)),
                }
            }
            Metal(albedo, fuzz) => {
                let reflected = reflect(&ray.direction.to_unit(), &normal);
                let scattered = Ray::new(point, reflected + fuzz * Vec3::random_unit_vector());
                if &scattered.direction % normal > 0.0 {
                    ScatterRecord {
                        attenuation: albedo,
                        ray: Some(scattered),
                    }
                } else {
                    ScatterRecord {
                        attenuation: albedo,
                        ray: None,
                    }
                }
            }
            Dielectric(attenuation, ref_idx) => {
                let ref_ratio = if front_face { 1.0 / *ref_idx } else { *ref_idx };
                let unit_dir = ray.direction.to_unit();
                let cos_theta = f32::min(-unit_dir.dot(&normal), 1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let direction = if ref_ratio * sin_theta > 1.0
                    || random::<f32>() < schlick(cos_theta, ref_ratio)
                {
                    reflect(&unit_dir, &normal)
                } else {
                    refract(&unit_dir, &normal, ref_ratio)
                };

                ScatterRecord {
                    attenuation: &attenuation,
                    ray: Some(Ray::new(point, direction)),
                }
            }
        }
    }
}
