use crate::renderer::vector3d::{Point, Vector3D};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3D,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

pub fn dot(first: Vector3D, second: Vector3D) -> f64 {
    first.x * second.x + first.y * second.y + first.z * second.z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_at() {
        let ray = Ray {
            origin: Point {
                x: 1.0,
                y: 2.0,
                z: 4.0,
            },
            direction: Vector3D {
                x: 1.0,
                y: 2.0,
                z: 6.0,
            },
        };
        let point = ray.at(5.0);
        assert_eq!(point.x, 6.0);
        assert_eq!(point.y, 12.0);
        assert_eq!(point.z, 34.0);
    }

    #[test]
    fn test_dot() {
        let first = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let second = Vector3D {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let dot_product = dot(first, second);
        assert_eq!(dot_product, 32.0);
    }
}