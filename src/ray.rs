use crate::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: Option<f64>) -> Ray {
        let unwrapped_time: f64;
        match time {
            Some(var) => unwrapped_time = var,
            None => unwrapped_time = 0.0
        }

        Ray {
            origin,
            direction,
            time: unwrapped_time
        }
    } 

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
}