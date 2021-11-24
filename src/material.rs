use crate::sample::*;
use crate::vec::*;

use Material::*;

const INVIS: Vec3 = Vec3::new(1.0, 1.0, 1.0);
pub const INVIS_MAT: Material = Invis;
const NO_SCATTER: ScatterRecord = ScatterRecord {
    attenuation: &INVIS,
    ray: None,
};

const SUN_COLOR: Vec3 = Vec3::new(1.0, 0.9, 0.95);

#[derive(Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, Float),
    Dielectric(Vec3, Float),
    NoScatter(Vec3),
    Invis,
    Sun,
    BiasedLambertian(Vec3),
}

pub struct ScatterRecord<'a> {
    pub attenuation: &'a Vec3,
    pub ray: Option<Ray>,
}

impl Material {
    pub fn scatter(&self, ray: &Ray, point: Vec3, normal: Option<Vec3>) -> ScatterRecord {
        match normal {
            None => NO_SCATTER,
            Some(normal) => match self {
                Lambertian(albedo) => {
                    let target = normal + sample_sphere();
                    ScatterRecord {
                        attenuation: albedo,
                        ray: Some(Ray::new(point, target)),
                    }
                }
                BiasedLambertian(albedo) => {
                    let target = normal + sample_sphere() * 0.10;
                    ScatterRecord {
                        attenuation: albedo,
                        ray: Some(Ray::new(point, target)),
                    }
                }
                Metal(albedo, fuzz) => {
                    let reflected = reflect(&ray.direction.to_unit(), &normal);
                    let scattered = Ray::new(point, reflected + fuzz * sample_sphere());
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
                    let reflected = reflect(&ray.direction, &normal);
                    let dotdir = &ray.direction % &normal;
                    let invref_idx = 1.0 / ref_idx;
                    let (outward_normal, nint, cosine) = if dotdir > 0.0 {
                        (
                            normal.negate(),
                            ref_idx,
                            ref_idx * dotdir / ray.direction.norm(),
                        )
                    } else {
                        (normal, &invref_idx, -dotdir / ray.direction.norm())
                    };

                    let direction = match refract(&ray.direction, &outward_normal, *nint) {
                        Some(refracted) => {
                            if random() < schlick(cosine, *ref_idx) {
                                reflected
                            } else {
                                refracted
                            }
                        }
                        None => reflected,
                    };

                    ScatterRecord {
                        attenuation: &attenuation,
                        ray: Some(Ray::new(point, direction)),
                    }
                }
                NoScatter(glow) => ScatterRecord {
                    attenuation: &glow,
                    ray: None,
                },
                Invis => ScatterRecord {
                    attenuation: &INVIS,
                    ray: None,
                },
                Sun => ScatterRecord {
                    attenuation: &SUN_COLOR,
                    ray: None,
                },
            },
        }
    }
}
