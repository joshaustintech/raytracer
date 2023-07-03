use crate::renderer::hit::HitRecord;
use crate::renderer::material::Material;
use crate::renderer::ray::{dot, Ray};
use crate::renderer::vector3d::{Color, unit_vector, Vector3D};

#[derive(Copy, Clone, Default)]
pub(crate) struct Metal {
    pub(crate) albedo: Color
}

fn reflect(vector: Vector3D, normal: Vector3D) -> Vector3D {
    vector - 2.0 * dot(vector, normal) * normal
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord
    ) -> Option<(Color, Ray)> {
        let reflected = reflect(unit_vector(ray.direction), hit_record.normal);
        let scattered = Ray {
            origin: hit_record.point,
            direction: reflected
        };

        if dot(scattered.direction, hit_record.normal) > 0.0 {
            return Some((self.albedo, scattered));
        }
        None
    }
}