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
use crate::renderer::vector3d::{Color, Point};

pub fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut hit_record = HitRecord::default();
    if world.hit(ray, 0.0, f64::INFINITY, &mut hit_record) {
        return 0.5 * (hit_record.normal + Color { x: 1.0, y: 1.0, z: 1.0 });
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
    let samples_per_pixel = 256;

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
        if y % 10 == 0 {
            println!("Scanlines remaining: {}", y);
        }
        for x in 0..width {
            let mut pixel_color = Color { x: 0.0, y: 0.0, z: 0.0 };
            for _ in 0..samples_per_pixel {
                let u = (x as f64 + random::<f64>()) / (width - 1) as f64;
                let v = (y as f64 + random::<f64>()) / (height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world);
            }
            render.push_str(&write_color(&pixel_color, samples_per_pixel));
        }
    }
    return render;
}