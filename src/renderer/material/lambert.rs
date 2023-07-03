use crate::renderer::hit::HitRecord;
use crate::renderer::material::Material;
use crate::renderer::ray::Ray;
use crate::renderer::vector3d::{Color, random_unit_vector};

#[derive(Copy, Clone, Default)]
pub(crate) struct Lambert {
    pub albedo: Color
}

impl Material for Lambert {
    fn scatter(
        &self,
        _: &Ray,
        hit_record: &HitRecord
    ) -> Option<(Color, Ray)> {

        let mut scatter_direction = hit_record.normal + random_unit_vector();

        // catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        Some((self.albedo, Ray {
            origin: hit_record.point,
            direction: scatter_direction,
        }))
    }
}


