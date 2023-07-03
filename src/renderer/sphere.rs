use std::rc::Rc;
use crate::renderer::hit::{Hittable, HitRecord};
use crate::renderer::material::Material;
use crate::renderer::ray::{dot, Ray};
use crate::renderer::vector3d::Point;

pub(crate) struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Rc<dyn Material>
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let object_center = ray.origin - self.center;
        let a: f64 = ray.direction.length_squared();
        let half_b: f64 = dot(object_center, ray.direction);
        let c = object_center.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discriminant_sqrt) / a;
        if root < t_min || t_max < root {
            root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let root_ray = ray.at(root);
        let outward_normal = (root_ray - self.center) / self.radius;

        let mut hit_record = HitRecord {
            point: root_ray,
            normal: outward_normal,
            material: self.material.clone(),
            t: root,
            front_face: false
        };
        hit_record.set_face_normal(ray, outward_normal);
        Some(hit_record)
    }
}