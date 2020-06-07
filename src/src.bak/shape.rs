use crate::vec::*;
use crate::material::*;

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub pos: Float,
    pub material: &'a Material
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: Float,
    pub material: Material,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let oc = &ray.origin - &self.center;
        let a = &ray.direction % &ray.direction;
        let b = &oc % &ray.direction;
        let c = (&oc % &oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (a * c);

        if discriminant < 0.0 {
            return None;
        }

        let temp = (-b - discriminant.sqrt()) / a;
        let point = ray.point_at(temp);
        let normal = ((&point - &self.center) / self.radius).to_unit();

        if temp < t_max && temp > t_min {
            return Some(HitRecord { point, normal, pos: temp, material: &self.material});
        }

        let temp = (-b + discriminant.sqrt()) / a;
        let point = ray.point_at(temp);
        let normal = ((&point - &self.center) / self.radius).to_unit();

        if temp < t_max && temp > t_min {
            return Some(HitRecord { point, normal, pos: temp, material: &self.material});
        }

        None
    }
}

pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Hitable for Plane {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let div = &ray.direction % &self.normal;
        if div == 0.0 {
            return None;
        }

        let d = (&self.origin - &ray.origin) % &self.normal / div;

        if d < t_min || d > t_max {
            return None;
        }

        Some(HitRecord {
            point: ray.point_at(d),
            normal: self.normal.clone(),
            pos: d,
            material: &self.material
        })
    }
}

pub struct HitableGroup {
    pub planes: Vec<Plane>,
    pub spheres: Vec<Sphere>,
}

impl Hitable for HitableGroup {

    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {

        let mut record = None;
        let mut closest = t_max;

        for item in &self.spheres {
            let intersect = item.hit(ray, t_min, closest);
            match intersect {
                None => (),
                Some(newhit) => {
                    closest = newhit.pos;
                    record = Some(newhit);
                }
            }
        }

        for item in &self.planes {
            let intersect = item.hit(ray, t_min, closest);
            match intersect {
                None => (),
                Some(newhit) => {
                    closest = newhit.pos;
                    record = Some(newhit);
                }
            }
        }

        record
    }

}
