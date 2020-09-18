use crate::material::*;
use crate::vec::*;

use crate::shape::KDTree::*;

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Option<Vec3>,
    pub pos: Float,
    pub material: &'a Material,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

pub trait Boxable {
    fn get_bbox(&self) -> AABB;
}

impl<T: Boxable> Boxable for Vec<T> {
    fn get_bbox(&self) -> AABB {
        self.iter()
            .map(|i| i.get_bbox())
            .fold_first(|a, b| a.absorb(&b))
            .unwrap() // Don't call w/ empty vectors
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: Float,
    pub material: Material,
}

impl Boxable for Sphere {
    fn get_bbox(&self) -> AABB {
        let radius = self.radius;
        AABB {
            min: Vec3::new(
                self.center.x - radius,
                self.center.y - radius,
                self.center.z - radius,
            ),
            max: Vec3::new(
                self.center.x + radius,
                self.center.y + radius,
                self.center.z + radius,
            ),
        }
    }
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
            return Some(HitRecord {
                point,
                normal: Some(normal),
                pos: temp,
                material: &self.material,
            });
        }

        let temp = (-b + discriminant.sqrt()) / a;
        let point = ray.point_at(temp);
        let normal = ((&point - &self.center) / self.radius).to_unit();

        if temp < t_max && temp > t_min {
            return Some(HitRecord {
                point,
                normal: Some(normal),
                pos: temp,
                material: &self.material,
            });
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
            normal: Some(self.normal.clone()),
            pos: d,
            material: &self.material,
        })
    }
}

#[derive(Clone)]
pub struct AABB {
    pub max: Vec3,
    pub min: Vec3,
}

impl AABB {
    fn absorb(&self, other: &AABB) -> AABB {
        AABB {
            max: self.max.elem_max(&other.max),
            min: self.min.elem_min(&other.min),
        }
    }
}

impl Boxable for AABB {
    fn get_bbox(&self) -> AABB {
        self.clone()
    }
}

impl Hitable for AABB {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let mut imin = t_min;
        let mut imax = t_max;

        for normal in &Vec3::cardinal_directions() {
            let div = &ray.direction % normal;
            let p = (&self.min - &ray.origin) % normal / div;
            let q = (&self.max - &ray.origin) % normal / div;

            imin = imin.max(p.min(q));
            imax = imax.min(q.max(p));
        }

        if imin < imax {
            Some(HitRecord {
                point: ray.point_at(imin),
                normal: Some(Vec3::new(0.0, 1.0, 0.0)),
                pos: imin,
                material: &INVIS_MAT,
            })
        } else {
            None
        }
    }
}

pub enum KDTree<T> {
    Leaf {
        bbox: AABB,
        items: Vec<T>,
    },
    Node {
        bbox: AABB,
        left: Box<KDTree<T>>,
        right: Box<KDTree<T>>,
    },
    Empty,
}

impl<T> Boxable for KDTree<T> {
    fn get_bbox(&self) -> AABB {
        match self {
            Empty => panic!(),
            Leaf { bbox, .. } => bbox.clone(),
            Node { bbox, .. } => bbox.clone(),
        }
    }
}

impl<T: Hitable> Hitable for KDTree<T> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        match self {
            Empty => None,
            Leaf { bbox, items } => {
                if bbox.hit(ray, t_min, t_max).is_none() {
                    return None;
                }

                let mut record = None;
                let mut closest = t_max;

                for item in items {
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
            Node { bbox, left, right } => {
                if bbox.hit(ray, t_min, t_max).is_none() {
                    return None;
                }

                let lhit = left.hit(ray, t_min, t_max);
                let rhit = right.hit(ray, t_min, t_max);
                match lhit {
                    None => rhit,
                    Some(hit) => match rhit {
                        None => Some(hit),
                        Some(ohit) => {
                            if ohit.pos < hit.pos {
                                Some(ohit)
                            } else {
                                Some(hit)
                            }
                        }
                    },
                }
            }
        }
    }
}

impl<T: Boxable + std::clone::Clone> KDTree<T> {
    pub fn new(items: Vec<T>) -> KDTree<T> {
        if items.len() == 0 {
            return Empty;
        }

        let bbox = items.get_bbox();

        // TODO find right constant
        if items.len() < 3 {
            return Leaf { bbox, items };
        }

        let diag = &bbox.max - &bbox.min;
        let splitdir = diag.longest_dimension();
        let splitval = (bbox.max.get(splitdir) + bbox.min.get(splitdir)) / 2.0;
        let rights: Vec<T> = items
            .iter()
            .cloned()
            .filter(|i| i.get_bbox().min.get(splitdir) < splitval)
            .collect();
        let lefts: Vec<T> = items
            .iter()
            .cloned()
            .filter(|i| i.get_bbox().max.get(splitdir) >= splitval)
            .collect();

        if rights.len() == items.len() || lefts.len() == items.len() {
            return Leaf { bbox, items };
        }

        Node {
            bbox,
            left: Box::new(KDTree::new(lefts)),
            right: Box::new(KDTree::new(rights)),
        }
    }
}

pub struct HitableGroup {
    pub planes: Vec<Plane>,
    pub spheres: KDTree<Sphere>,
}

impl Hitable for HitableGroup {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let mut record = None;
        let mut closest = t_max;

        let intersect = self.spheres.hit(ray, t_min, closest);
        match intersect {
            None => (),
            Some(newhit) => {
                closest = newhit.pos;
                record = Some(newhit);
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
