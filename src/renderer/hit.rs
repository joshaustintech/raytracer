use std::rc::Rc;
use crate::renderer::material::Material;
use crate::renderer::ray::{dot, Ray};
use crate::renderer::vector3d::{Point, Vector3D};

pub struct HitRecord {
    pub point: Point,
    pub normal: Vector3D,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3D) {
        self.front_face = dot(ray.direction, outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }

}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}