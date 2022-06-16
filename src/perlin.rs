use crate::Vec3;
use crate::utilities::*;

pub struct Perlin {
    rand_vec: Vec<Vec3>,
    x_perms: Vec<usize>,
    y_perms: Vec<usize>,
    z_perms: Vec<usize>
}

const POINT_COUNT: u32 = 256;

impl Perlin {
    pub fn new() -> Perlin {
        let mut rand_vec: Vec<Vec3> = Vec::new();
        for _i in 0..POINT_COUNT {
            rand_vec.push(Vec3::random_unit_vector());
        }

        Perlin {
            rand_vec: rand_vec,
            x_perms: Perlin::generate_perm(),
            y_perms: Perlin::generate_perm(),
            z_perms: Perlin::generate_perm(),
        }
    }

    // a sum of multiple frequencies
    pub fn turbulence(&self, point: &Vec3, depth: i32) -> f64 {
        let mut accumulate = 0.0;
        let mut previous_point = *point;
        let mut weight = 1.0;

        for _i in 0..depth {
            accumulate += weight * self.noise(&previous_point);
            weight *= 0.5;
            previous_point = previous_point * 2.0;
        }
        accumulate.abs()
    }

    pub fn noise(&self, point: &Vec3) -> f64 {
        // basic 'blocky' perlin noise
        // sign matters so cast to int and not usize directly
        // let i = (4.0 * point.x()) as i64 & 255;
        // let j = (4.0 * point.y()) as i64 & 255;
        // let k = (4.0 * point.z()) as i64 & 255;
        // let index = self.x_perms[i as usize] ^ self.y_perms[j as usize] ^ self.z_perms[k as usize];        
        // self.rand_float[index]

        let mut u = point.x() - point.x().floor();
        let mut v = point.y() - point.y().floor();
        let mut w = point.z() - point.z().floor();
        // apply Hermitian smoothing
        // u = u * u * (3.0 - 2.0 * u);
        // v = v * v * (3.0 - 2.0 * v);
        // w = w * w * (3.0 - 2.0 * w);

        let i = point.x().floor() as i64;
        let j = point.y().floor() as i64;
        let k = point.z().floor() as i64;
        let mut values = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let a = self.x_perms[((i + di) & 255) as usize];
                    let b = self.x_perms[((j + dj) & 255) as usize];
                    let c = self.x_perms[((k + dk) & 255) as usize];
                    values[di as usize][dj as usize][dk as usize] = self.rand_vec[a ^ b ^ c]
                }
            }
        }
        Perlin::trilinear_interpolation(&values, u, v, w)
    }

    fn trilinear_interpolation(values: &[[[Vec3; 2]; 2];2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u)) *
                             (j as f64 * v + (1.0 - j as f64) * (1.0 - v)) *
                             (k as f64 * w + (1.0 - k as f64) * (1.0 - w)) *
                             values[i][j][k].dot_product(&weight);
                }
            }
        }
        accum
    }

    fn generate_perm() -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        for i in 0..POINT_COUNT {
            result.push(i as usize);
        }
        Perlin::permute(&mut result, POINT_COUNT);
        result
    }

    fn permute(perms: &mut Vec<usize>, n: u32) {
        for i in (0..n as usize).rev() {
            let target = random_int_in_range(0, (i + 1) as u32);
            perms.swap(i, target as usize);
        }
    }
}