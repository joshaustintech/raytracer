use crate::renderer::hit::{Hittable, HitRecord};
use crate::renderer::ray::{dot, Ray};
use crate::renderer::vector3d::Point;

pub(crate) struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let object_center = ray.origin - self.center;
        let a: f64 = ray.direction.length_squared();
        let half_b: f64 = dot(object_center, ray.direction);
        let c = object_center.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discriminant_sqrt) / a;
        if root < t_min || t_max < root {
            root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);
        true
    }
}