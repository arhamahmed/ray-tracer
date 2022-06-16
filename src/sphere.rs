use crate::Vec3;
use crate::Ray;
use crate::hittable::*;
use crate::material::*;
use crate::aabb::AABB;
use crate::utilities::PI;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material
        }
    }

    // convert the point from cartesian to spherical coordinates
    // point: a point on a unit sphere centered at the origin
    pub fn get_sphere_uv(point: Vec3) -> (f64, f64) {
        // represents the angle from the south pole upwards (-Y to +Y)
        let theta = f64::acos(point.y() * -1.0);
        // represents the value from -X to +X (-X -> +Z -> +X -> -Z -> -X)
        let phi = (-1.0 * point.z()).atan2(point.x()) + PI;
        // returning (u, v) where:
        // u is [0, 1], value of angle around y axis
        // v is [0, 1] value of angle from south to north pole (-Y to +Y)
        (phi / (2.0 * PI), theta / PI)
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

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin_to_center = ray.origin - self.center;
        // using an (optimized) quadratic formula (let b = 2h so the '2a' becomes an 'a')
        // to see if ray intersects sphere
        let a: f64 = ray.direction.length_squared();
        let half_b: f64 = origin_to_center.dot_product(&ray.direction);
        let c: f64 = origin_to_center.length_squared() - self.radius * self.radius;
        let root = Sphere::get_first_root_in_range(a, half_b, c, t_min, t_max);
        match root {
            None => return None,
            Some(_) => ()
        }

        let t = root.unwrap();
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let (u, v): (f64, f64) = Sphere::get_sphere_uv(outward_normal);
        let mut record = HitRecord::new(point, outward_normal, t, u, v, false, &self.material);
        // adjust normal so that it's always pointing away from the ray
        record.set_face_normal(ray, &outward_normal);
        Some(record)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let radius = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(self.center - radius, self.center + radius))
    }
}