use rand::Rng;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64{
    degrees * PI / 180.0
}

pub fn random_int_in_range(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)}

pub fn random_float() -> f64 {
    // Rng::gen_range(0.0..1.0,)
    rand::random::<f64>()
}

pub fn random_float_in_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

// fix given x to be in [min, max]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min
    }
    if x > max {
        return max
    }
    return x
}