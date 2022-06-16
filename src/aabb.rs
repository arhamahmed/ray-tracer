use crate::Vec3;
use crate::Ray;

// axis-aligned bounding boxes.
// idea is to optimize the check for a ray intersecting a list of
// hittables by finding a 'box' that contains an object then checking if a=that box
// is hit. only iff a box (aabb) is hit, then what's inside can be checked
// which may be an object, or another aabb and so on.
// ideally, these aabb's are grouped into a hierarchical tree to achieve
// logarithmic performance
#[derive(Copy, Clone)]
pub struct AABB {
    // the minimum bounds of each plane (x, y, z)
    pub minimum: Vec3,
    // the maximum bounds of each plane (x, y, z)
    pub maximum: Vec3
}

impl AABB {
    pub fn new(minimum: Vec3, maximum: Vec3) -> AABB {
        AABB {
            minimum,
            maximum
        }
    }

    pub fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> bool {
        // check intersection in each dimension
        self.computeIntersection(ray, min_t, max_t, 0) && 
        self.computeIntersection(ray, min_t, max_t, 1) &&
        self.computeIntersection(ray, min_t, max_t, 2)
    }

    // combines two given boxes
    pub fn surrounding_box(first: AABB, second: AABB) -> AABB {
        let small: Vec3 = Vec3::new(
            first.minimum.x().min(second.minimum.x()),
            first.minimum.y().min(second.minimum.y()),
            first.minimum.z().min(second.minimum.z()),
        );
        let big: Vec3 = Vec3::new(
            first.maximum.x().max(second.maximum.x()),
            first.maximum.y().max(second.maximum.y()),
            first.maximum.z().max(second.maximum.z()),
        );
        AABB::new(small, big)
    }

    /*
        reference: https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-box-intersection

        this is much easier to see when visualized, strongly recommend the above
        link. it doesn't help that this method is using an optimized hit method
        from Andrew Kensler (https://www.researchgate.net/publication/232618536_Optimizing_Ray-Triangle_Intersection_via_Automated_Search)
        
        essentially we're checking the start and end of where a ray intersects a box,
        if at all
    
               t0______t1 (the point in the line, e.g. the t in y(t) = mt + b)
                |      | 
         -------|      |-------> ray
                |______|

        in the above 2D example we only happen to care about the x dimension
        where we want to make sure the min and max t the ray intersects are
        in the range of the given min and max t. in the case of a diagonal line
        just the intersection values for the x plane is not enough, the t values
        for in the y plane must intersect as well. in 3D, the same goes for z

        ray: ray to check intersection in min_t and max_t
        min_t: minimum t to check for intersection
        max_t: maximum t to check for intersection
        dimension: 0 -> x, 1 -> y, 2 -> z
    */
    fn computeIntersection(&self, ray: &Ray, min_t: f64, max_t: f64, dimension: u32) -> bool {
        let mut t0: f64;
        let mut t1: f64;
        let inv_d: f64;
        if dimension == 0 {
            inv_d = 1.0 / ray.direction.x();
            t0 = (self.minimum.x() - ray.origin.x()) * inv_d;
            t1 = (self.maximum.x() - ray.origin.x()) * inv_d;
        } else if dimension == 1 {
            inv_d = 1.0 / ray.direction.y();
            t0 = (self.minimum.y() - ray.origin.y()) * inv_d;
            t1 = (self.maximum.y() - ray.origin.y()) * inv_d;
        } else {
            inv_d = 1.0 / ray.direction.z();
            t0 = (self.minimum.z() - ray.origin.z()) * inv_d;
            t1 = (self.maximum.z() - ray.origin.z()) * inv_d;
        }

        let orig_t0: f64 = t0;
        if inv_d < 0.0 {
            t0 = t1;
            t1 = orig_t0;
        }

        let min_computed_t: f64;
        let max_computed_t: f64;

        if t0 > min_t {
            min_computed_t = t0;
        } else {
            min_computed_t = min_t;
        }

        if t1 < max_t {
            max_computed_t = t1;
        } else {
            max_computed_t = max_t;
        }

        if max_computed_t <= min_computed_t {
            false
        } else {
            true
        }
    }
}