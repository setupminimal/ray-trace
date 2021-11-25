use crate::shapes::Hitable;
use crate::{Color, Ray};

pub struct Scene {
    objects: Vec<Box<dyn Hitable + Sync>>,
}

impl Scene {
    pub fn empty() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn add<O>(&mut self, o: O)
    where
        O: Hitable + Sync + 'static,
    {
        self.objects.push(Box::new(o))
    }

    pub fn color(&self, r: &Ray, depth: usize) -> Color {
        // Too many bounces
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        // TODO refactor for tail calls
        //   - Then add blackness limit

        // t_min value makes sure floating point issues don't cause
        // shadow acne
        if let Some(rec) = self.objects.hit(r, 0.001, f32::INFINITY) {
            let scatter =
                rec.mat
                    .scatter(r, r.point_at(rec.loc), rec.normal.clone(), rec.front_face);
            if let Some(mut ray) = scatter.ray {
                if ray.direction.near_zero() {
                    ray = Ray::new(rec.point.clone(), rec.normal.clone());
                }
                return scatter.attenuation * self.color(&ray, depth - 1);
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }
        }

        let dir = r.direction.to_unit();
        let t = 0.5 * (dir.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
