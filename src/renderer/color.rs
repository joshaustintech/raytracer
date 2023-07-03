use crate::renderer::hit::Hittable;
use crate::renderer::ray::Ray;
use crate::renderer::util::clamp;
use crate::renderer::vector3d::{Color, unit_vector};

pub fn write_color(pixel_color: &Color, samples: i32) -> String {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    let ir = (256.0 * clamp(r, 0.0, 0.999)) as u32;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as u32;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as u32;

    format!("{} {} {}\n", ir, ig, ib)
}

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color { x: 0.0, y: 0.0, z: 0.0 };
    }
    return match world.hit(ray, 0.001, f64::INFINITY) {
        Some(hit_record) => {
            match hit_record.material.scatter(ray, &hit_record) {
                Some((attenuation, scattered)) => attenuation * ray_color(&scattered, world, depth - 1),
                None => Color { x: 0.0, y: 0.0, z: 0.0 }
            }
        },
        None => {
            let unit_direction = unit_vector(ray.direction);
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 as f64 - t) * Color { x: 1.0, y: 1.0, z: 1.0 } + t * Color { x: 0.5, y: 0.7, z: 1.0 }
        }
    };
}