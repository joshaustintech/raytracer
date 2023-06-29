use std::ops;

#[derive(Debug)]
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

impl ops::Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        self + -other
    }
}

impl ops::Mul for Vector3D {
    type Output = Vector3D;

    fn mul(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Div for Vector3D {
    type Output = Vector3D;

    fn div(self, other: Self) -> Self::Output {
        Vector3D {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl ToString for Vector3D {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.x, self.y, self.z)
    }
}

impl Vector3D {
    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn cross(first: &Vector3D, second: &Vector3D) -> Vector3D {
        Vector3D {
            x: first.y * second.z - first.z * second.y,
            y: first.z * second.x - first.x * second.z,
            z: first.x * second.y - first.y * second.x,
        }
    }
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
    // Note this useful idiom: importing names from outer (for mod tests) scope.
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
    fn test_vector3d_subtract() {
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
    fn test_vector3d_multiply() {
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
    fn test_vector3d_divide() {
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
    fn test_vector3d_cross() {
        let first = Vector3D {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        let second = Vector3D {
            x: 5.0,
            y: 6.0,
            z: 7.0,
        };
        let cross = Vector3D::cross(&first, &second);
        assert_eq!(cross.x, -3.0);
        assert_eq!(cross.y, 6.0);
        assert_eq!(cross.z, -3.0);
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