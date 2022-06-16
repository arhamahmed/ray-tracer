use crate::vec3::*;
use crate::perlin::Perlin;

pub trait Texture {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color;
}

pub struct SolidTexture {
    color_val: Color
}

impl SolidTexture {
    pub fn new(color: Color) -> SolidTexture {
        SolidTexture {
            color_val: color
        }
    }
}

impl Texture for SolidTexture {
    // u, v are surface coordinates
    fn value(&self, _u: f64, _v: f64, _point: &Vec3) -> Color {
        self.color_val
    }
}

pub struct CheckeredTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>
}

impl CheckeredTexture {
    pub fn new_with_texture(odd: impl Texture + 'static, even: impl Texture + 'static) -> CheckeredTexture {
        CheckeredTexture {
            odd: Box::new(odd),
            even: Box::new(even)
        }
    }

    pub fn new_with_solid(odd: Color, even: Color) -> CheckeredTexture {
        CheckeredTexture {
            odd: Box::new(SolidTexture::new(odd)),
            even: Box::new(SolidTexture::new(even))
        }
    }
}

impl Texture for CheckeredTexture {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color {
        let sines = f64::sin(10.0 * point.x()) * f64::sin(10.0 * point.y()) * f64::sin(10.0 * point.z());
        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)            
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    frequency: f64
}

impl NoiseTexture {
    pub fn new(frequency: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            frequency
        }
    }    
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, point: &Vec3) -> Color {
        // correlate turbulences with a sine function to give a 'marble-like' texture
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + f64::sin(self.frequency * point.z() + 10.0 * self.noise.turbulence(point, 7)))
        // this gives a "net-like" texture
        // Color::new(1.0, 1.0, 1.0) * self.noise.turbulence(&(*point * self.frequency), 7)
        // this gives a kind of smoothened blocky texture
        // Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(&(*point * self.frequency)))
    }
}