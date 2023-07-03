pub mod vector3d;
pub mod ray;
pub mod hit;
pub mod sphere;
pub mod world;
pub mod util;
pub mod camera;
pub mod color;
pub mod material;

use std::rc::Rc;
use rand::{Rng, random};

use crate::renderer::camera::Camera;
use crate::renderer::color::{ray_color, write_color};
use crate::renderer::world::HittableList;
use crate::renderer::sphere::Sphere;
use crate::renderer::vector3d::{Color, Point};
use crate::renderer::material::{Material, metal::Metal};
use crate::renderer::material::lambert::Lambert;

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
    let samples_per_pixel = 50;
    let depth = 25;

    // World
    let mut world = HittableList::new();


    for x in 1..10 {
        for z in 1..5 {
            let mut rng = rand::thread_rng();
            let choose_mat: Rc<dyn Material> = match rng.gen_range(1..4) {
                1 => Rc::new(Metal { albedo: Color { x: 0.05, y: 0.05, z: 0.05 } }),
                2 => Rc::new(Metal { albedo: Color { x: 0.7, y: 0.7, z: 0.7 } }),
                _ => Rc::new(Metal { albedo: Color { x: 0.0, y: 0.0, z: 0.33 } }),
            };
            let coord_x = -3.0 + (0.6 * x as f64);
            let coord_z = -1.2 - (0.5 * z as f64);
            world.add(Box::new(Sphere {
                center: Point { x: coord_x, y: -0.75, z: coord_z },
                radius: 0.25,
                material: choose_mat,
            }));
        }
    }
    world.add(Box::new(Sphere {
        center: Point { x: 0.0, y: -101.0, z: 0.0 },
        radius: 100.0,
        material: Rc::new(Lambert { albedo: Color { x: 0.0, y: 0.7, z: 0.0 } }),
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