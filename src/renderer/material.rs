pub mod lambert;
pub mod metal;

use crate::renderer::hit::HitRecord;
use crate::renderer::ray::Ray;
use crate::renderer::vector3d::Color;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord
    ) -> Option<(Color, Ray)>;
}