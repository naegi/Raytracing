use crate::{
    color,
    math::vec::{RgbAsVec3Ext, Vec3AsRgbExt},
    ray::Ray,
    renderer::{RayResult, World},
    shape::IntersectionResult,
    timed_scope_accumulate,
};

use super::Integrator;

pub struct BasicIntegrator {
    pub max_depth: u32,
}

impl Integrator for BasicIntegrator {
    fn ray_cast(&self, world: &World, ray: Ray, depth: u32) -> RayResult {
        let mut rng = rand::thread_rng();
        if depth == self.max_depth {
            return RayResult::default();
        }

        // Prevent auto intersection
        let ray = Ray::new_with_range(ray.origin, ray.direction, 0.01..ray.bounds.1);

        let isect = timed_scope_accumulate!("Intersection", || {
            world.objects.intersection_full(ray)
        });
        let IntersectionResult::Intersection(record) = isect else  {
            return self.sky_ray(world, ray);
        };

        // On material hit
        let material = &world.materials[record.local_info.material.0].material;
        let scattered = material.scatter(ray, &record.local_info, &mut rng);

        let (color, depth) = if let Some(ray_out) = scattered.ray_out {
            let ray_result = self.ray_cast(world, ray_out, depth + 1);
            (ray_result.color, ray_result.ray_depth + 1.0)
        } else {
            (color::linear::WHITE, depth as f32)
        };

        let color = (color.vec() * scattered.albedo.vec()).rgb();

        RayResult {
            normal: record.local_info.normal,
            position: record.local_info.pos,
            albedo: scattered.albedo,
            color,
            z: record.t,
            ray_depth: depth,
            samples_accumulated: 1,
        }
    }
}
