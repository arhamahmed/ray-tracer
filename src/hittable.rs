use crate::Vec3;
use crate::Ray;
use crate::material::Material;
use crate::aabb::AABB;

pub struct HitRecord<'a> {
    // where ray hits a Hittable
    pub point: Vec3,
    // normal protuding from the surface of the Hittable at intersection
    pub normal: Vec3,
    // how far away intersection was from 'center'
    pub t: f64,
    // coordinate on the surface of the object
    pub u: f64,
    // coordinate on the surface of the object
    pub v: f64,
    // true if ray hits the outside surface
    pub front_face: bool,
    // the material of the object
    pub material: &'a Material
}

impl<'a> HitRecord<'a> {
    pub fn new(point: Vec3, normal: Vec3, t: f64, u: f64, v: f64, front_face: bool, material: &Material) -> HitRecord {
        HitRecord{
            point,
            normal,
            t,
            u,
            v,
            front_face,
            material
        }
    }

    // updates whether a ray hits an object from the front or back.
    // note: the normal, by design, will always point away from the ray
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot_product(outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            // normal goes 'inward'
            self.normal = *outward_normal * -1.0;
        }
    }
}

pub trait Hittable {
    // returns if a given ray hits an object between a ray, updates the HitRecord.
    // note we're returning a record instead of updating references in place (pain)
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    // compute the aabb (box) that contains a hittable object
    // note we're returning a record instead of updating references in place (pain)
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;

    // for BVH, can clone the Hittable if we dont wanna pass around references
    // fn clone(&self) -> Box<dyn Hittable>;
}