use std::f32::INFINITY;

use glam::Vec3;

use crate::{
    material::MaterialDescriptor,
    math::vec::{RefrReflVecExt, RgbAsVec3Ext, Vec3AsRgbExt},
    ray::Ray,
    renderer::{RayResult, World},
    shape::IntersectionResult,
};

use super::Integrator;

pub struct WhittedIntegrator {
    pub max_depth: u32,
}

impl Integrator for WhittedIntegrator {
    fn ray_cast(&self, world: &World, ray: Ray, depth: u32) -> RayResult {
        if depth == self.max_depth {
            return RayResult::default();
        }

        let mut ray_depth = (depth + 1) as f32;

        let IntersectionResult::Intersection(intersection)= world.objects.intersection_full(ray) else {return self.sky_ray(world, ray);};

        let MaterialDescriptor { ref material, .. } =
            world.materials[intersection.local_info.material.0];
        let position = intersection.local_info.pos;
        let normal = intersection.local_info.normal;

        let ambiant = 'emissive: {
            let Some(emissive) = material.emissive() else {break 'emissive Vec3::ZERO};

            emissive
        };
        let (albedo, diffuse) = 'diffuse: {
            let Some(albedo) = material.diffuse() else {break 'diffuse (Vec3::ZERO, Vec3::ZERO);};
            let mut diffuse = Vec3::ZERO;

            for light_pos in world.lights.iter() {
                // cast shadow ray to check light visibility
                if true {
                    let light_dir = *light_pos - position;

                    let attenuation = normal.dot(light_dir).clamp(0.0, 1.0);
                    diffuse += albedo * attenuation;
                }
            }

            (albedo, diffuse)
        };

        let transmission = 'transmission: {
            let Some((ior, transmission_color)) = material.transmission() else {break 'transmission Vec3::ZERO;};
            let Some(refracted) = ray.direction.refract(-intersection.local_info.normal, ior) else {break 'transmission Vec3::ZERO;};

            let refracted_ray = Ray::new_with_range(position, refracted, 0.01..INFINITY);
            let refracted_ray_result = self.ray_cast(world, refracted_ray, depth + 1);
            ray_depth = ray_depth.max(refracted_ray_result.ray_depth);

            transmission_color * refracted_ray_result.color.vec()
        };

        let reflection = 'reflection: {
            let Some(reflection_color) = material.reflection() else {break 'reflection Vec3::ZERO;};
            let reflected = ray.direction.reflect(intersection.local_info.normal);

            let reflected_ray = Ray::new_with_range(position, reflected, 0.01..INFINITY);
            let reflected_ray_result = self.ray_cast(world, reflected_ray, depth + 1);
            ray_depth = ray_depth.max(reflected_ray_result.ray_depth);

            reflection_color * reflected_ray_result.color.vec()
        };

        let color = (ambiant + diffuse + transmission + reflection).rgb();
        RayResult {
            normal,
            position,
            albedo: albedo.rgb(),
            color,
            z: intersection.t,
            ray_depth,
            samples_accumulated: 1,
        }
    }
}
