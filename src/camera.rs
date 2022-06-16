use crate::vec3::Vec3;
use crate::Ray;
use crate::utilities::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    plane_outward: Vec3, // w
    plane_horizontal: Vec3, //u
    plane_vertical: Vec3, // v
    min_time: f64,
    max_time: f64
}

impl Camera {
    // lookfrom is where camera is positioned
    // lookat is where the camera is pointed at.
    // vertical_up tells us where 'up' is to determine the camera tilt
    // focus_dist is the distance from the lens (camera) to the focus plane (target); not the same as focal length
    pub fn new(lookfrom: Vec3, lookat: Vec3, vertical_up: Vec3, vertical_fov: f64, aspect_ratio: f64, 
        aperture: f64, focus_dist: f64, min_time: f64, max_time: f64) -> Camera { 
        let theta = degrees_to_radians(vertical_fov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // w is a vector opposite to the direction the camera is looking in
        let plane_outward = (lookfrom - lookat).unit_vector(); // w
        // u and v are perpendicular vectors on the plane the camera is on
        let plane_horizontal = vertical_up.cross_product(&plane_outward).unit_vector(); // u
        let plane_vertical = plane_outward.cross_product(&plane_horizontal); // v

        let origin = lookfrom;
        let horizontal:Vec3 = plane_horizontal * viewport_width * focus_dist;
        let vertical:Vec3 = plane_vertical * viewport_height * focus_dist;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            plane_outward,
            plane_horizontal,
            plane_vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - plane_outward * focus_dist,
            lens_radius,
            min_time,
            max_time
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let ray_dir = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.plane_horizontal * ray_dir.x() + self.plane_vertical * ray_dir.y();
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            time: random_float_in_range(self.min_time, self.max_time)
        }
    }
}