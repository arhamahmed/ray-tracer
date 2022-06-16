use crate::Vec3;
use crate::texture::*;
use crate::Color;
use crate::Ray;
use crate::HitRecord;
use crate::utilities::random_float;

pub enum Material {
    // diffuse (matte). albedo is the degree of reflection
    Lambertian{albedo: Box<dyn Texture>},
    // Lambertian{albedo: Box<dyn Texture>},
    // metal (shiny). albedo is the degree of reflection, fuzz is how much to blur
    Metal{albedo: Vec3, fuzz: f64},
    // glass. index of refraction adjusts how much to bend light
    Dielectric{index_of_refraction: f64}
}

pub struct Scattering {
    attenuation: Vec3,
    scattered: Ray
}

impl Scattering {
    pub fn new(attenuation: Vec3, scattered: Ray) -> Scattering {
        Scattering {
            attenuation,
            scattered
        }
    }

    pub fn attenuation(&self) -> Vec3 {
        Vec3::new(self.attenuation.x(), self.attenuation.y(), self.attenuation.z())
    }

    pub fn scattered(&self) -> Ray {
        Ray::new(self.scattered.origin, self.scattered.direction, Some(self.scattered.time))
    }
}

// depending on the angle a ray hits a [shiny/dielectric] surface e.g. a mirror
// the outbound ray may become reflected
fn reflectance(cosine: f64, reflective_index: f64) -> f64 {
    // Schlick's approximation
    let r0 = (1.0 - reflective_index) / (1.0 + reflective_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl MaterialScattering for Material {
    fn scatter(&self, inc_ray: &Ray, record: &HitRecord) -> Option<Scattering> {
        match self {
            // implement diffusion (matte material) via rays bouncing off into random directions.
            Self::Lambertian{albedo} => {
                let mut scatter_direction = record.normal + Vec3::random_unit_vector();

                // case where random vector could cancel out the normal
                if scatter_direction.near_zero() {
                    scatter_direction = record.normal;
                }

                let scattered = Ray::new(record.point, scatter_direction, Some(inc_ray.time));
                // let attenuation = Color::new(albedo.x(), albedo.y(), albedo.z());
                // let attenuation = Color::new(record.t, record.u, record.v); //
                let attenuation = albedo.value(record.u, record.v, &record.point);
                return Some(Scattering::new(attenuation, scattered))
            },
            // with metal surfaces, rays are reflected off the surface of the object
            Self::Metal{albedo, fuzz} => {
                let reflected = Vec3::reflect(&inc_ray.direction.unit_vector(), &record.normal);
                // without the fuzz and random vector it would look like glass
                let scattered = Ray::new(record.point, reflected + Vec3::random_in_unit_sphere() * (*fuzz), Some(inc_ray.time));
                let attenuation = Color::new(albedo.x(), albedo.y(), albedo.z());
                let dot = scattered.direction.dot_product(&record.normal);
                if dot > 0.0 {
                    Some(Scattering::new(attenuation, scattered))
                } else {
                    None
                }
            },
            // glass material
            Self::Dielectric{index_of_refraction} => {
                let attenuation = Color::new(1.0, 1.0, 1.0);
                let mut refraction_ratio = *index_of_refraction;
                if record.front_face {
                    refraction_ratio = 1.0 / (*index_of_refraction);
                }
                let unit_direction = inc_ray.direction.unit_vector();
                let cos_theta = (unit_direction * -1.0).dot_product(&record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
                let direction: Vec3;

                // total internal reflection
                if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_float() {
                    direction = Vec3::reflect(&unit_direction, &record.normal);
                } else {
                    direction = Vec3::refract(&unit_direction, &record.normal, refraction_ratio);
                }

                Some(Scattering::new(attenuation, Ray::new(record.point, direction, Some(inc_ray.time))))
            }
        }
    }
}

pub trait MaterialScattering {
    fn scatter(&self, inc_ray: &Ray, record: &HitRecord) -> Option<Scattering>;
}