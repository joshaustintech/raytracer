use crate::renderer::ray::Ray;
use crate::renderer::vector3d::{Point, Vector3D};

pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,

    origin: Point,
    lower_left_corner: Point,
    horizontal: Vector3D,
    vertical: Vector3D,

}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let origin = Point { x: 0.0, y: 0.0, z: 0.0 };
        let horizontal = Vector3D { x: viewport_width, y: 0.0, z: 0.0 };
        let vertical = Vector3D { x: 0.0, y: viewport_height, z: 0.0 };
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vector3D { x: 0.0, y: 0.0, z: focal_length };

        Self {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + (u * self.horizontal) + (v * self.vertical) - self.origin,
        }
    }

}