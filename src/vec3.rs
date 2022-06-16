use std::ops::*;
use crate::utilities::*;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot_product(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_product(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z) / self.length()
    }

    pub fn write_colour(&self, samples_per_pixel: u64) {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        let scale = 1.0 / samples_per_pixel as f64;

        // perform gamma correction because of how light is perceived/displayed
        // (adjusts ligting due to the non-linearity of light perception)
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();
        
        println!("{0} {1} {2}", 256.0 * clamp(r, 0.0, 0.999), 256.0 * clamp(g, 0.0, 0.999), 256.0 * clamp(b, 0.0, 0.999));
    }

    pub fn equal_to(&self, second: &Vec3) -> bool {
        self.x == second. x && self.y == second.y && self.z == second.z
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_float(), random_float(), random_float())
    }

    pub fn random_in_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(random_float_in_range(min, max), random_float_in_range(min, max), random_float_in_range(min, max))
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let point = Vec3::new(
                random_float_in_range(-1.0, 1.0),
                random_float_in_range(-1.0, 1.0),
                0.0
            );

            if point.length_squared() < 1.0 {
                return point;
            }
        }
    }

    // basic diffusion (this + normal)
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let vec = Vec3::random_in_range(-1.0, 1.0);
            if vec.length_squared() < 1.0 {
                return vec;
            }
        }
    }

    // "hack" to approximate lambertian reflection
    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    // hemispherical scattering
    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        // in the same hemisphere as the normal
        if in_unit_sphere.dot_product(normal) > 0.0 {
            in_unit_sphere
        } else {
            in_unit_sphere * -1.0
        }
    }

    pub fn near_zero(&self) -> bool {
        let min = 1e-8;
        self.x().abs() < min && self.y().abs() < min && self.z().abs() < min
    }

    // v       ^
    //  \  n  /
    //   \ | /
    // ___\ /___
    // vector v is reflected off the surface, n is the norma
    pub fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
        let x = vector.dot_product(normal) * 2.0;
        let y = *normal * 2.0;
        let reflected_vector = *vector - *normal * x;
        Vec3::new(reflected_vector.x(), reflected_vector.y(), reflected_vector.z())
    }

    // using snell's law
    pub fn refract(uv: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta: f64 = uv.dot_product(&(*normal * -1.0)).min(1.0);
        let r_out_perp: Vec3 = (*uv + (*normal * cos_theta)) * etai_over_etat;
        let r_out_paralel: Vec3 = *normal * ((1.0 - r_out_perp.length_squared()).abs().sqrt()) * -1.0;
        r_out_perp + r_out_paralel
    }
}

// vec3 + vec3
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x() + other.x(),
            y: self.y() + other.y(),
            z: self.z() + other.z()
        }
    }
}

// vec3 - vec3
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x() - other.x(),
            y: self.y() - other.y(),
            z: self.z() - other.z()
        }
    }
}

// vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, factor: f64) -> Vec3 {
        Vec3 {
            x: self.x() * factor,
            y: self.y() * factor,
            z: self.z() * factor
        }
    }
}

// vec3 * vec3
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3 {
            x: self.x() * vector.x(),
            y: self.y() * vector.y(),
            z: self.z() * vector.z()
        }
    }
}

// vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, factor: f64) -> Vec3 {
        Vec3 {
            x: self.x() / factor,
            y: self.y() / factor,
            z: self.z() / factor
        }
    }
}

pub type Color = Vec3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let expected = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        };

        assert!(expected.equal_to(&Vec3::new(0.0, 0.0, 0.0)));
    }
    
    #[test]
    fn test_cross_product() {
        let first = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0
        };
        let second = Vec3 {
            x: 2.0,
            y: 3.0,
            z: 4.0
        };
        let expected = Vec3 {
            x: -1.0,
            y: 2.0,
            z: -1.0
        };
        let actual = first.cross_product(&second);
        assert!(expected.equal_to(&actual));
    }
}
