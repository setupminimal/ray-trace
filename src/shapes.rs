use crate::material::Material;
use crate::{Ray, Vec3};

// TODO KDTree and AABB

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub loc: f32,
    pub front_face: bool,
    pub mat: &'a Material,
}

impl<'b> HitRecord<'b> {
    fn new<'a>(
        loc: f32,
        out_normal: Vec3,
        point: Vec3,
        r: &Ray,
        mat: &'a Material,
    ) -> HitRecord<'a> {
        let front_face = r.direction.dot(&out_normal) < 0.0;

        HitRecord {
            loc,
            point,
            front_face,
            normal: if front_face {
                out_normal
            } else {
                -1.0 * out_normal
            },
            mat,
        }
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub mat: Material,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = &r.origin - &self.center;
        let a = r.direction.dot(&r.direction);
        let half_b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let disc = half_b * half_b - a * c;
        if disc < 0.0 {
            None
        } else {
            let sqrtd = disc.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return None;
                }
            }

            let point = r.point_at(root);

            Some(HitRecord::new(
                root,
                (&point - &self.center) / self.radius,
                point,
                r,
                &self.mat,
            ))
        }
    }
}

impl<T> Hitable for Vec<T>
where
    T: Hitable,
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest = t_max;
        for object in self {
            match object.hit(r, t_min, closest) {
                None => {}
                Some(hr) => {
                    closest = hr.loc;
                    rec = Some(hr);
                }
            }
        }
        rec
    }
}

impl Hitable for Box<dyn Hitable + Sync> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        (&**self).hit(r, t_min, t_max)
    }
}
