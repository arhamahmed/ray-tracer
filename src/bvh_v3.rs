use crate::Ray;
use crate::aabb::AABB;
use crate::hittable::*;
use crate::utilities::random_int_in_range;
use std::cmp::Ordering;

// Bounding Volume Hierarchy.
// construct a hierarchy of aabb boxes. this improves performance of
// the 'hit' method (if a ray hits an object) by constructing a tree of
// boxes, where the parent hit means one of the children were hit. finding
// out if a box was hit is a fast computation, and traversing the tree is a
// logarithmic operation(s) as opposed to checking a list of objects repeatedly
// for every ray encountered
pub enum BVH {
    // left/right are Hittable's because it could refer to either:
    // - another BVH node
    // - an object (leaf node)
    Leaf(Box<dyn Hittable>),
    Branch {
        left: Box<BVH>,
        right: Box<BVH>,
        bounding_box: AABB
    }
}

impl BVH {
    // ideally the children have smaller boxes, and each subtree is 
    // equally distributed. implement a simple strategy:
    // 1. randomly pick an axis
    // 2. sort
    // 3. take half of the sorted for the left and right subtrees

    // not using HittableList for the list type because:
    // 1 - don't need to use its methods since the elements implement them too
    // 2 - list.objects makes the caller take ownership, then retrieving an
    //     an element in objects causes a double borrow
    pub fn construct(mut list: Vec<Box<dyn Hittable>>, t0: f64, t1: f64) -> Self {
        let axis = random_int_in_range(0, 3);
        let span = list.len();
        let left;
        let right;
        if span == 0 {
            panic!("Cannot have 0 objects in list during BVH construction");
        }
        if span == 1 {
            return BVH::Leaf(list.pop().unwrap())
        } else {
            // TODO: can optimize by splitting on the axis with the largest span
            list.sort_by(|a, b| {
                let box1 = a.bounding_box(t0, t1);
                let box2 = b.bounding_box(t0, t1);
                match(box1, box2) {
                    (Some(q), Some(u)) => {
                        let left_val: f64;
                        let right_val: f64;
                        match axis {
                            0 => {
                                left_val = q.minimum.x();
                                right_val = u.minimum.x()
                            },
                            1 => {
                                left_val = q.minimum.y();
                                right_val = u.minimum.y()
                            },
                            _ => {
                                left_val = q.minimum.z();
                                right_val = u.minimum.z()
                            }
                        }
                        if left_val < right_val {
                            Ordering::Less
                        } else if left_val == right_val {
                            Ordering::Equal
                        } else {
                            Ordering::Greater
                        }
                    },
                    (Some(_q), None) => panic!("No bounding box in BVH node"),
                    (None, Some(_q)) => panic!("No bounding box in BVH node"),
                    (None, None) => panic!("No bounding box in BVH node"),
                }
            });

            right = Box::new(BVH::construct(list.drain(span / 2..).collect(), t0, t1));
            left = Box::new(BVH::construct(list, t0, t1));
        }

        let left_box = left.bounding_box(t0, t1);
        let right_box = right.bounding_box(t0, t1);
        let result_box: AABB;

        match(left_box, right_box) {
            (Some(q), Some(u)) => {
                result_box = AABB::surrounding_box(q, u);
            },
            (Some(_q), None) => panic!("No bounding box in BVH node"),
            (None, Some(_q)) => panic!("No bounding box in BVH node"),
            (None, None) => panic!("No bounding box in BVH node"),
        }

        BVH::Branch {
            left: left,
            right: right,
            bounding_box: result_box
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            BVH::Leaf(t) => {
                t.hit(ray, t_min, t_max)
            },
            BVH::Branch {left, right, bounding_box} => {
                // only recursively check if the current box is even hit
                if !bounding_box.hit(ray, t_min, t_max) {
                    return None
                }

                let left_hit = left.hit(ray, t_min, t_max);
                let mut end = t_max;
                // don't unnecessarily search more area than needed
                if let Some(hit) = &left_hit {
                    end = hit.t;
                }

                let right_hit = right.hit(ray, t_min, end);

                // return the closer object hit
                match (left_hit, right_hit) {
                    (None, None) => None,
                    (None, Some(y)) => Some(y),
                    (Some(x), None) => Some(x),
                    (Some(x), Some(y)) => {
                        if x.t < y.t {
                            Some(x)
                        } else {
                            Some(y)
                        }
                    }
                }
            }
        }
    }

    // might need to fix this, surroung on left and right?
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        match self {
            BVH::Leaf(t) => {
                t.bounding_box(t0, t1)
            },
            BVH::Branch {left: _, right: _, bounding_box} => {
                Some(*bounding_box)
            }
        }
    }
}