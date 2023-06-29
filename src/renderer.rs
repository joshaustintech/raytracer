pub mod vector3d;
pub mod ray;

use crate::renderer::ray::{Ray, dot};
use crate::renderer::vector3d::{Color, Point, Vector3D};

pub fn hit_sphere(center: Point, radius: f64, ray: &Ray) -> f64 {
    let object_center = ray.origin - center;
    let a: f64 = dot(ray.direction, ray.direction);
    let b: f64 = 2.0 * dot(object_center, ray.direction);
    let c = dot(object_center, object_center) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

pub fn ray_color(ray: &Ray) -> Color {
    let mut t = hit_sphere(Point { x: 0.0, y: 0.0, z: -1.0 }, 0.5, ray);
    if t > 0.0 {
        let normal = ray.at(t) - Point { x: 0.0, y: 0.0, z: -1.0 };
        return 0.5 * Color { x: normal.x + 1.0, y: normal.y + 1.0, z: normal.z + 1.0 };
    }
    t = 0.5 * (ray.direction.y + 1.0);
    1.0 as f64 - t * Color { x: 1.0, y: 1.0, z: 1.0 } + t * Color { x: 0.5, y: 0.7, z: 1.0 }
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

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point { x: 0.0, y: 0.0, z: 0.0 };
    let horizontal = Vector3D { x: viewport_width, y: 0.0, z: 0.0 };
    let vertical = Vector3D { x: 0.0, y: viewport_height, z: 0.0 };
    let lower_left_corner = origin -
        (horizontal / (2.0 as f64)) -
        (vertical / (2.0 as f64)) -
        Vector3D { x: 0.0, y: 0.0, z: focal_length };

    // Render
    let mut render = String::new();
    create_header(&mut render, width, height);

    for y in (0..height).rev() {
        for x in 0..width {
            let u = x as f64 / (width - 1) as f64;
            let v = y as f64 / (height - 1) as f64;
            let ray = Ray {
                origin,
                direction: lower_left_corner + (horizontal * u) + (vertical * v) - origin,
            };
            let color = ray_color(&ray);
            render.push_str(color.get_color().as_str());
            render.push_str("\n");
        }

        if y % (height / 4) == 0 {
            println!("{}% complete", 100 - (y * 100 / height));
        }

    }
    return render;
}