use std::ops;
use rand::Rng;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Vector3D {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        self + -other
    }
}

impl ops::Sub<Vector3D> for f64 {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self - other.x,
            y: self - other.y,
            z: self - other.z,
        }
    }
}

impl ops::Mul<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn mul(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, other: f64) -> Vector3D {
        Vector3D {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::Mul<Vector3D> for f64 {
    type Output = Vector3D;

    fn mul(self, other: Vector3D) -> Vector3D {
        other * self
    }
}

impl ops::Div<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn div(self, other: Self) -> Self::Output {
        Vector3D {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl ops::Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, other: f64) -> Self::Output {
        self * (1.0 / other)
    }
}

impl ToString for Vector3D {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.x, self.y, self.z)
    }
}

impl Vector3D {

    pub(crate) fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[allow(dead_code)]
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

fn random(min: f64, max: f64) -> Vector3D {
    let mut rng = rand::thread_rng();
    Vector3D {
        x: rng.gen_range(min..max+1.0),
        y: rng.gen_range(min..max+1.0),
        z: rng.gen_range(min..max+1.0),
    }
}

pub fn random_in_unit_sphere() -> Vector3D {
    loop {
        let p = random(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn unit_vector(v: Vector3D) -> Vector3D {
    v / v.length()
}

pub fn random_unit_vector() -> Vector3D {
    unit_vector(random_in_unit_sphere())
}

pub use Vector3D as Color;

impl Color {
    pub fn get_color(&self) -> String {
        format!("{} {} {}",
                (255.999 * self.x) as u32,
                (255.999 * self.y) as u32,
                (255.999 * self.z) as u32
        )
    }
}

pub use Vector3D as Point;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3d_negative() {
        let vector = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let negative = -vector;
        assert_eq!(negative.x, -1.0);
        assert_eq!(negative.y, -2.0);
        assert_eq!(negative.z, -3.0);
    }

    #[test]
    fn test_vector3d_add() {
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
        let sum = first + second;
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);
    }

    #[test]
    fn test_vector3d_subtract_vector3d() {
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
        let difference = first - second;
        assert_eq!(difference.x, -3.0);
        assert_eq!(difference.y, -3.0);
        assert_eq!(difference.z, -3.0);
    }

    #[test]
    fn test_f64_subtract_vector3d() {
        let first = 1.0 as f64;
        let second = Vector3D {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let difference = first - second;
        assert_eq!(difference.x, -3.0);
        assert_eq!(difference.y, -4.0);
        assert_eq!(difference.z, -5.0);
    }

    #[test]
    fn test_vector3d_multiply_vector3d() {
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
        let product = first * second;
        assert_eq!(product.x, 4.0);
        assert_eq!(product.y, 10.0);
        assert_eq!(product.z, 18.0);
    }

    #[test]
    fn test_vector3d_multiply_f64() {
        let vector = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let product = vector * 2.0;
        assert_eq!(product.x, 2.0);
        assert_eq!(product.y, 4.0);
        assert_eq!(product.z, 6.0);
    }

    #[test]
    fn test_f64_multiply_vector3d() {
        let vector = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let product = 2.0 * vector;
        assert_eq!(product.x, 2.0);
        assert_eq!(product.y, 4.0);
        assert_eq!(product.z, 6.0);
    }

    #[test]
    fn test_vector3d_divide_vector32() {
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
        let quotient = first / second;
        assert_eq!(quotient.x, 0.25);
        assert_eq!(quotient.y, 0.4);
        assert_eq!(quotient.z, 0.5);
    }

    #[test]
    fn test_vector3d_divide_f64() {
        let vector = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let quotient = vector / 2.0;
        assert_eq!(quotient.x, 0.5);
        assert_eq!(quotient.y, 1.0);
        assert_eq!(quotient.z, 1.5);
    }

    #[test]
    fn test_vector3d_to_string() {
        let vector = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(vector.to_string(), "1 2 3");
    }

    #[test]
    fn test_vector3d_length_squared() {
        let vector = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(vector.length_squared(), 14.0);
    }

    #[test]
    fn test_vector3d_length() {
        let vector = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(vector.length(), vector.length_squared().sqrt());
    }

    #[test]
    fn test_color_get_color() {
        let color = Color {
            x: 0.5,
            y: 0.6,
            z: 0.7,
        };
        assert_eq!(color.get_color(), "127 153 179");
    }

}