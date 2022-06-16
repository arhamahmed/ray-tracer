use crate::Ray;
use crate::aabb::AABB;
use crate::hittable::*;
use crate::HittableList;
use crate::utilities::random_int_in_range;
use std::cmp::Ordering;

// Bounding Volume Hierarchy.
// construct a hierarchy of aabb boxes. this improves performance of
// the 'hit' method (if a ray hits an object) by constructing a tree of
// boxes, where the parent hit means one of the children were hit. finding
// out if a box was hit is a fast computation, and traversing the tree is a
// logarithmic operation(s) as opposed to checking a list of objects repeatedly
// for every ray encountered
pub struct BVH {
    // left/right are Hittable's because it could refer to either:
    // - another BVH node
    // - an object (leaf node)
    left: Option<Box<BVH>>,
    right: Option<Box<BVH>>,
    value: Option<Box<dyn Hittable>>,
    bounding_box: AABB

    // ^ should work but weird edge cases have to be handled
    // e.g. left/right/value all not null, value/left not null but right is... etc
    //. it should only be left/right/value not null or left/right null and not value
}

impl BVH {
    pub fn construct(list: &mut HittableList, t0: f64, t1: f64) -> BVH {
        // ideally the children have smaller boxes, and each subtree is 
        // equally distributed. implement a simple strategy:
        // 1. randomly pick an axis
        // 2. sort
        // 3. take half of the sorted for the left and right subtrees
        BVH::construct_partial(list, 0, list.objects.len(), t0, t1)
    }

    pub fn construct_partial(list: &mut HittableList, start: usize, end: usize, t0: f64, t1: f64) -> BVH {
        let axis = random_int_in_range(0, 2);
        let span = end - start;
        let mut left: Option<Box<BVH>>;
        let mut right: Option<Box<BVH>>;
        let value: Option<Box<dyn Hittable>>;
        if span == 1 {
            left = None;
            right = None;
            value = Some(list.objects[start]);
        } else if span == 2 {
            left = Some(Box::new(BVH::construct_partial(list, start, start, t0, t1)));
            left = Some(Box::new(BVH::construct_partial(list, end, end, t0, t1)));
            value = None;
        } else {
            list.objects.sort_by(|a, b| {
                let box1 = a.bounding_box(t0, t1);
                let box2 = b.bounding_box(t0, t1);
                // fix
                match(box1, box2) {
                    (Some(q), Some(u)) => {
                        if q.minimum.x() < u.minimum.x() {
                            Ordering::Less
                        } else {
                            Ordering::Equal
                        }
                    },
                    (Some(q), None) => Ordering::Equal,
                    (None, Some(q)) => Ordering::Equal,
                    (None, None) => Ordering::Equal,
                }
            });

            let mid = start + span / 2;
            // fix
            left = Some(Box::new(BVH::construct_partial(list, start, mid, t0, t1)));
            right = Some(Box::new(BVH::construct_partial(list, mid, end, t0, t1)));
            value = None;
        }
        let left_box = left.unwrap().bounding_box(t0, t1);
        let right_box = right.unwrap().bounding_box(t0, t1);
        let result_box: AABB;

        match(left_box, right_box) {
            (Some(q), Some(u)) => {
                result_box = AABB::surrounding_box(q, u);
            },
            (Some(_q), None) => panic!("No bounding box in BVH node"),
            (None, Some(_q)) => panic!("No bounding box in BVH node"),
            (None, None) => panic!("No bounding box in BVH node"),
        }

        BVH {
            left,
            right,
            value: None,
            bounding_box: result_box
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None
        }

        let left_hit: Option<HitRecord> = None;// = self.left.hit(ray, t_min, t_max);
        let right_hit: Option<HitRecord> = None;//= self.right.hit(ray, t_min, t_max);
        // return something as long as one side was hit
        match (left_hit, right_hit) {
            (None, None) => return None,
            (None, Some(y)) => return Some(y),
            (Some(x), None) => return Some(x),
            (Some(x), Some(_y)) => return Some(x)
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.bounding_box)
    }
}