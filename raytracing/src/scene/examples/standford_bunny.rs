use glam::{Quat, Vec3};

use crate::{
    color::Rgb,
    loader::ObjLoaderExt,
    material::{texture, Emit, Gooch},
    math::transform::Transform,
    scene::Scene,
};

pub struct StandfordBunnyScene;
impl From<StandfordBunnyScene> for Scene {
    fn from(_: StandfordBunnyScene) -> Self {
        let mut scene = Scene::new(Emit {
            texture: Box::new(texture::Uniform(Rgb::from_array([0.3, 0.3, 0.3]))),
        });

        let default_material = scene.insert_material(
            Some("Goosh - Default".to_string()),
            Gooch {
                diffuse: Rgb::from_array([1.0, 0., 0.]),
                smoothness: 20.0,
                light_dir: Vec3::new(-1.0, -1.0, 0.0),
                yellow: Rgb::from_array([0.8, 0.8, 0.0]),
                blue: Rgb::from_array([0.0, 0.0, 0.8]),
            },
        );

        scene.load_obj(
            "./obj/standford_bunny.obj",
            Transform {
                translation: Vec3::new(0.2, -0.3, -0.5),
                scale: Vec3::splat(4.0),
                rot: Quat::IDENTITY,
            },
            default_material,
        );

        scene
    }
}
