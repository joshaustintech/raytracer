pub mod vector3d;
pub mod ray;
pub mod hit;
pub mod sphere;
pub mod world;
pub mod util;
pub mod camera;
pub mod color;

use rand::random;

use crate::renderer::camera::Camera;
use crate::renderer::color::write_color;
use crate::renderer::hit::{HitRecord, Hittable};
use crate::renderer::world::HittableList;
use crate::renderer::sphere::Sphere;
use crate::renderer::ray::Ray;
use crate::renderer::vector3d::{Color, Point, random_in_unit_sphere};

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color { x: 0.0, y: 0.0, z: 0.0 };
    }
    let mut hit_record = HitRecord::default();
    if world.hit(ray, 0.001, f64::INFINITY, &mut hit_record) {
        let target = hit_record.point + hit_record.normal + random_in_unit_sphere();
        let new_depth = depth - 1;
        return 0.5 * ray_color(&Ray { origin: hit_record.point, direction: target - hit_record.point }, world, new_depth);
    }
    let t = 0.5 * (ray.direction.y + 1.0);
    (1.0 as f64 - t) * Color { x: 1.0, y: 1.0, z: 1.0 } + t * Color { x: 0.5, y: 0.7, z: 1.0 }
}

fn create_header(render: &mut String, width: u32, height: u32) {
    println!("Width: {}px, Height: {}px", width, height);
    render.push_str("P3\n");
    render.push_str(&width.to_string());
    render.push_str(" ");
    render.push_str(&height.to_string());
    render.push_str("\n255\n");
}

pub fn render(width: u32) -> String {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let height = (width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 64;
    let depth = 24;

    // World
    let mut world = HittableList::new();

    world.add(Box::new(Sphere {
        center: Point { x: 0.125, y: 0.0, z: -1.0 },
        radius: 0.5,
    }));

    world.add(Box::new(Sphere {
        center: Point { x: 0.0, y: -100.5, z: -1.0 },
        radius: 100.0,
    }));

    // Camera
    let camera = Camera::new();

    // Render
    let mut render = String::new();
    create_header(&mut render, width, height);

    for y in (0..height).rev() {
        for x in 0..width {
            println!("Pixel {} of {}, {} scanlines remain", x, width, y);
            let mut pixel_color = Color { x: 0.0, y: 0.0, z: 0.0 };
            for _ in 0..samples_per_pixel {
                let u = (x as f64 + random::<f64>()) / (width - 1) as f64;
                let v = (y as f64 + random::<f64>()) / (height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world, depth);
            }
            render.push_str(&write_color(&pixel_color, samples_per_pixel));
        }
    }
    return render;
}