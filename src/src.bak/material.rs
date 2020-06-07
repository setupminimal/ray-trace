use crate::vec::*;
use crate::sample::*;

use Material::*;

pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, Float),
    Dielectric(Vec3, Float),
    NoScatter(Vec3),
}

pub struct ScatterRecord<'a> {
    pub attenuation: &'a Vec3,
    pub ray: Option<Ray>,
}

impl Material {
    pub fn scatter(&self, ray: &Ray, point: Vec3, normal: Vec3) -> ScatterRecord {
        match self {
            Lambertian(albedo) => {
                let target = normal + sample_sphere();
                ScatterRecord { attenuation: albedo,
                                ray: Some(Ray { origin: point,
                                                direction: target }) }
            },
            Metal(albedo, fuzz) => {
                let reflected = reflect(&ray.direction.to_unit(), &normal);
                let scattered = Ray { origin: point,
                                      direction: reflected + fuzz * sample_sphere() };
                if &scattered.direction % normal > 0.0 {
                    ScatterRecord { attenuation: albedo,
                                    ray: Some(scattered) }
                } else {
                    ScatterRecord { attenuation: albedo,
                                    ray: None }
                }
            },
            Dielectric(attenuation, ref_idx) => {
                let reflected = reflect(&ray.direction, &normal);
                let dotdir = &ray.direction % &normal;
                let invref_idx = 1.0 / ref_idx;
                let (outward_normal, nint, cosine) =
                    if dotdir > 0.0 {
                        (normal.negate(),
                         ref_idx,
                         ref_idx * dotdir / ray.direction.norm())
                    } else {
                        (normal,
                        &invref_idx,
                        -dotdir / ray.direction.norm())
                    };

                let direction = match refract(&ray.direction, &outward_normal, *nint) {
                    Some(refracted) => if random() < schlick(cosine, *ref_idx) { reflected } else { refracted },
                    None => reflected

                };

                ScatterRecord {
                    attenuation: &attenuation,
                    ray: Some(Ray { origin: point, direction })
                }
            },
            NoScatter(glow) => {
                ScatterRecord {
                    attenuation: &glow,
                    ray: None
                }
            }
        }
    }
}
