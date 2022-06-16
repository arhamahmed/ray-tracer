use crate::Sphere;
use crate::Vec3;
use crate::Ray;
use crate::Material;
use crate::Hittable;
use crate::HitRecord;
use crate::aabb::AABB;

// sphere linearly moves from center0 at time0 to center1 at time1
pub struct MovingSphere {
    center_0: Vec3,
    time_0: f64,
    center_1: Vec3,
    time_1: f64,
    radius: f64,
    material: Material
}

impl MovingSphere {
    pub fn new(center_0: Vec3, time_0: f64, center_1: Vec3, time_1: f64,
         radius: f64, material: Material) -> MovingSphere {
        MovingSphere {
            center_0,
            time_0,
            center_1,
            time_1,
            radius,
            material
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center_0 + (self.center_1 - self.center_0) * ((time - self.time_0) / (self.time_1 - self.time_0))
    }

    // using an (optimized) quadratic formula (let b = 2h so the '2a' becomes an 'a' etc.)
    // to see if ray intersects sphere
    fn get_first_root_in_range(a: f64, b: f64, c: f64, min: f64, max: f64) -> Option<f64> {
        let discriminant: f64 = b * b - a * c;
        // no roots
        if discriminant < 0.0 {
            return None
        }

        let discrim_sqrt = discriminant.sqrt();
        let root = (-b - discrim_sqrt) / a;

        if root > min && root < max {
            return Some(root)
        }

        let root = (-b + discrim_sqrt) / a;
        if root > min && root < max {
            return Some(root)
        }

        None
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin_to_center = ray.origin - self.center(ray.time);
        // using an (optimized) quadratic formula (let b = 2h so the '2a' becomes an 'a')
        // to see if ray intersects sphere
        let a: f64 = ray.direction.length_squared();
        let half_b: f64 = origin_to_center.dot_product(&ray.direction);
        let c: f64 = origin_to_center.length_squared() - self.radius * self.radius;
        let root = MovingSphere::get_first_root_in_range(a, half_b, c, t_min, t_max);
        match root {
            None => return None,
            Some(_) => ()
        }

        let t = root.unwrap();
        let point = ray.at(t);
        let outward_normal = (point - self.center(ray.time)) / self.radius;
        let (u, v): (f64, f64) = Sphere::get_sphere_uv(outward_normal);
        let mut record = HitRecord::new(point, outward_normal, t, u, v, false, &self.material);
        // adjust normal so that it's always pointing away from the ray
        record.set_face_normal(ray, &outward_normal);
        Some(record)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let radius = Vec3::new(self.radius, self.radius, self.radius);
        let t0_center = self.center(t0);
        let t1_center = self.center(t1);
        let first_box = AABB::new(t0_center - radius, t0_center + radius);
        let second_box = AABB::new(t1_center - radius, t1_center + radius);
        Some(AABB::surrounding_box(first_box, second_box))
    }
}