mod vec3;
mod ray;
mod sphere;
mod moving_sphere;
mod hittable;
mod hittable_list;
mod utilities;
mod camera;
mod material;
mod aabb;
mod bvh_v3;
mod texture;
mod perlin;

use vec3::*;
use sphere::Sphere;
use moving_sphere::MovingSphere;
use ray::Ray;
use hittable::*;
use hittable_list::HittableList;
use utilities::*;
use camera::Camera;
use material::*;
use bvh_v3::BVH;
use texture::*;
use perlin::Perlin;

// we shade the spere based on its normal (gives us orientation of lighting)
// e.g. if an object faces a light source it should be bright, dark if not
// e.g. if _|_ * (| is object, * is sun, _ is ground) how should | be shaded
fn ray_colour(ray: &Ray, world: &HittableList, depth: u64) -> Vec3 {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // see if ray intersects sphere so adjust color accordingly.
    // use 0.001 instead of 0 to correct for the 'shadow acne' problem:
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/ligth-and-shadows
    if let Some(record) = world.hit(ray, 0.001, INFINITY) {
        if let Some(scattering) = record.material.scatter(ray, &record) {
            return scattering.attenuation() * ray_colour(&scattering.scattered(), world, depth - 1);
        }

        return Color::new(0.0, 0.0, 0.0)
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // blue to white background gradient (t = 1 -> blue, t = 0 -> white)
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();
    let ground_albedo = Vec3::new(0.5, 0.5, 0.5);
    // ground
    world.add(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Material::Lambertian{albedo: Box::new(SolidTexture::new(ground_albedo))}));
    
    for a in -11..11 {
        for b in -11..11 {
            let mat_choice = random_float();
            let center = Vec3::new(a as f64 + 0.9 * random_float(), 0.2, b as f64 + 0.9 * random_float());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                // diffuse
                if mat_choice < 0.8 {
                    let albedo = Color::random() * Vec3::random();
                    world.add(Sphere::new(center, 0.2, Material::Lambertian{albedo: Box::new(SolidTexture::new(albedo))}));
                // metal
                } else if mat_choice < 0.95 {
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = random_float_in_range(0.0, 0.5);
                    world.add(Sphere::new(center, 0.2, Material::Metal{albedo: albedo, fuzz: fuzz}));
                // glass
                } else {
                    world.add(Sphere::new(center, 0.2, Material::Dielectric{index_of_refraction: 1.5}));
                }
            }
        }
    }

    // front glass sphere
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Material::Dielectric{index_of_refraction: 1.5}));
    let m_albedo = Color::new(0.4, 0.2, 0.1);
    // front matte sphere
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Material::Lambertian{albedo: Box::new(SolidTexture::new(m_albedo))}));
    let m2_albedo = Color::new(0.7, 0.6, 0.5);
    let m2_fuzz = 0.0;
    // front metal sphere
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Material::Metal{albedo: m2_albedo, fuzz: m2_fuzz}));

    world
}

fn basic_zoomed_in_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();

    let material_ground = Color::new(0.8, 0.8, 0.0);
    let material_center = Color::new(0.1, 0.2, 0.5);
    let material_right = Color::new(0.7, 0.6, 0.5);

    let white_green_checkered = CheckeredTexture::new_with_solid(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
    let ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Material::Lambertian{albedo: Box::new(white_green_checkered)});
    // let middle = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Material::Lambertian{albedo: material_center});
    // moving middle sphere
    let center_0 = Vec3::new(0.0, 0.0, -1.0);
    let center_1 = center_0 + Vec3::new(0.0, random_float_in_range(0.0, 0.5), 0.0);
    // let middle = MovingSphere::new(center_0, 0.0, center_1, 1.0, 0.5, Material::Lambertian{albedo: Box::new(SolidTexture::new(material_center))});
    let middle = Sphere::new(center_0, 0.5, Material::Lambertian{albedo: Box::new(SolidTexture::new(material_center))});
    // the 2 spheres below work together to make a hollow glass 'bubble'
    let left = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Material::Dielectric{index_of_refraction: 1.5});
    // note: negative radius doesn't change anything, however normal's point inward.
    // note: doesn't work properly with AABB/BVH because of the radius
    // let left_inner = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, Material::Dielectric{index_of_refraction: 1.5});
    let right = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Material::Metal{albedo: material_right, fuzz: 0.0});

    let mut y: Vec<Box<dyn Hittable>> = Vec::new();
    y.push(Box::new(ground));        // ground
    y.push(Box::new(middle));        // middle, matte sphere
    y.push(Box::new(left));          // left metal sphere
    // y.push(Box::new(left_inner));    // left metal sphere (inner)
    y.push(Box::new(right));         // right metal sphere
    world.add(BVH::construct(y, 0.0, 1.0));

    // world.add(ground);        // ground
    // world.add(middle);        // middle, matte sphere
    // world.add(left);          // left metal sphere
    // // world.add(left_inner);    // left metal sphere (inner)
    // world.add(right);         // right metal sphere

    world
}

fn perlin_noise() -> HittableList {
    let mut world: HittableList = HittableList::new();

    let perlin = Box::new(NoiseTexture::new(4.0));
    let perlin_sphere = Box::new(NoiseTexture::new(4.0));
    let ground = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Material::Lambertian{albedo: perlin});
    let sphere = Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Material::Lambertian{albedo: perlin_sphere});

    world.add(ground);
    world.add(sphere);
    // let mut y: Vec<Box<dyn Hittable>> = Vec::new();
    // y.push(Box::new(ground));
    // y.push(Box::new(sphere));
    // world.add(BVH::construct(y, 0.0, 1.0));

    world
}

fn checkered_spheres() -> HittableList {
    let mut world: HittableList = HittableList::new();
    let white = Color::new(0.2, 0.3, 0.1);
    let green = Color::new(0.9, 0.9, 0.9);
    let top_texture = Box::new(CheckeredTexture::new_with_solid(white, green));
    let bottom_texture = Box::new(CheckeredTexture::new_with_solid(white, green));
    let top_circle = Sphere::new(Vec3::new(0.0, -10.0, -1.0), 10.0, Material::Lambertian{albedo: bottom_texture});
    let bottom_circle = Sphere::new(Vec3::new(0.0, 10.0, -1.0), 10.0, Material::Lambertian{albedo: top_texture});
    world.add(bottom_circle);
    world.add(top_circle);
    world
}

pub struct ImageConfig {
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub image_height: i32,
    pub samples_per_pixel: u64,
    pub max_depth: u64
}

impl ImageConfig {
    pub fn new(aspect_ratio: f32, image_width: i32, samples_per_pixel: u64, max_depth: u64) -> ImageConfig {
        ImageConfig {
            aspect_ratio,
            image_width,
            image_height: (image_width as f32 / aspect_ratio) as i32,
            samples_per_pixel,
            max_depth
        }
    }
}

fn get_scene(number: usize) -> (ImageConfig, Camera, HittableList) {
    match number {
        // basic zoomed in scene
        0 => {
            let image = ImageConfig::new(16.0 / 9.0, 400, 10, 50);
            let lookfrom = Vec3::new(5.0, 2.0, 4.0);
            let lookat = Vec3::new(0.0, 0.0, -1.0);
            let vup = Vec3::new(0.0, 1.0, 0.0);
            let dist_to_focus = (lookfrom - lookat).length();
            let aperture = 0.1;
            let camera: Camera = Camera::new(lookfrom, lookat, vup, 20.0, image.aspect_ratio.into(), aperture, dist_to_focus, 0.0, 1.0);
            (image, camera, basic_zoomed_in_scene())
        },
        // 2 big checkered spheres
        1 => {
            let image = ImageConfig::new(16.0 / 9.0, 400, 10, 50);
            let lookfrom = Vec3::new(13.0, 2.0, 3.0);
            let lookat = Vec3::new(0.0, 0.0, 0.0);
            let vup = Vec3::new(0.0, 1.0, 0.0);
            let dist_to_focus = 10.0;
            let aperture = 0.0;
            let camera: Camera = Camera::new(lookfrom, lookat, vup, 20.0, image.aspect_ratio.into(), aperture, dist_to_focus, 0.0, 1.0);
            (image, camera, checkered_spheres())
        },
        // perlin noise
        2 => {
            let image = ImageConfig::new(16.0 / 9.0, 400, 100, 50);
            let lookfrom = Vec3::new(13.0, 2.0, 3.0);
            let lookat = Vec3::new(0.0, 0.0, 0.0);
            let vup = Vec3::new(0.0, 1.0, 0.0);
            let dist_to_focus = 10.0;
            let aperture = 0.0;
            let camera: Camera = Camera::new(lookfrom, lookat, vup, 20.0, image.aspect_ratio.into(), aperture, dist_to_focus, 0.0, 1.0);
            (image, camera, perlin_noise())
        }
        // random scene
        _ => {
            //                                           500 spp originally
            let image = ImageConfig::new(3.0 / 2.0, 1200, 10, 50);
            let lookfrom = Vec3::new(13.0, 2.0, 3.0);
            let lookat = Vec3::new(0.0, 0.0, 0.0);
            let vup = Vec3::new(0.0, 1.0, 0.0);
            let dist_to_focus = 10.0;
            let aperture = 0.1;
            let camera: Camera = Camera::new(lookfrom, lookat, vup, 20.0, image.aspect_ratio.into(), aperture, dist_to_focus, 0.0, 1.0);
            (image, camera, random_scene())
        }
    }
}

fn main() {
    let (image, camera, world): (ImageConfig, Camera, HittableList) = get_scene(0);
    println!("P3\n{0} {1}\n255", image.image_width, image.image_height);

    for j in (0..image.image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image.image_width {
            let mut pixel_colour = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..image.samples_per_pixel {
                let u = (i as f64 + random_float()) / (image.image_width - 1) as f64;
                let v = (j as f64 + random_float()) / (image.image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_colour = pixel_colour + ray_colour(&ray, &world, image.max_depth);
            }
            pixel_colour.write_colour(image.samples_per_pixel);
        }
    }
}
