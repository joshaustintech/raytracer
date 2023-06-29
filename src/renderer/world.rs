use crate::renderer::hit::{Hittable, HitRecord};
use crate::renderer::ray::Ray;

pub(crate) struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects = Vec::new();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *hit_record = temp_record;
            }
        }
        hit_anything
    }
}