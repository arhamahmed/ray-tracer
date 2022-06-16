use crate::hittable::*;
use crate::ray::Ray;
use crate::aabb::AABB;

pub struct HittableList {
    // "box" (put x trait into a fixed size container) Hittable because traits
    // aren't exactly the same as interfaces. traits don't have a fixed size
    pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new()
        }
    }
    pub fn add(&mut self, obj_to_add: impl Hittable + 'static) {
        self.objects.push(Box::new(obj_to_add))
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut result: Option<HitRecord> = None;

        for hittable_object in self.objects.iter() {
            // if there was a hit
            if let Some(hit) = hittable_object.hit(ray, t_min, closest_so_far) {
                // update result with the closest hit
                closest_so_far = hit.t;
                result = Some(hit);
            }
        }

        result
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.objects.len() == 0 {
            return None
        }

        let mut result_box: Option<AABB> = None;
        // let mut first_box = true;

        for hittable_object in self.objects.iter() {
            if let Some(output_box) = hittable_object.bounding_box(t0, t1) {
                // construct a box containing the entire list of objects
                match result_box {
                    // first box
                    None => {
                        result_box = Some(output_box);
                    },
                    Some(old_result) => {
                        result_box = Some(AABB::surrounding_box(old_result, output_box))
                    }
                }
                // if first_box {
                //     result_box = output_box;
                // } else {
                //     result_box = AABB::surrounding_box(result_box, output_box)
                // }
                // first_box = false;
            } else {
                return None
            }
        }

        result_box
    }
}