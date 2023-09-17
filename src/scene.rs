use crate::geometry::sphere::Sphere;
use crate::hit::{Hit, Hittable};
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::texture::checker_texture::CheckerTexture;
use crate::utils::random;
use crate::{Color, HittableList};
use std::sync::Arc;

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let checker = Box::new(CheckerTexture::from_color(
        0.32,
        color!(0.2, 0.3, 0.1),
        color!(0.9, 0.9, 0.9),
    ));
    let material_ground = Lambertian::textured(checker);
    world.add(Arc::new(Sphere::new(
        point!(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>(0.0..=1.0);
            let center = point!(
                a as f64 + 0.9 * random(0.0..=1.0),
                0.2,
                b as f64 + 0.9 * random(0.0..=1.0)
            );
            if (center - point!(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    let center2 = center + vec3!(0.0, random(0.0..=0.5), 0.0);
                    world.add(Arc::new(Sphere::moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::delimited(0.5..=1.0);
                    let fuzz = random(0.0..=0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material = Dielectric::new(1.5);
    world.add(Arc::new(Sphere::new(point!(0.0, 1.0, 0.0), 1.0, material)));

    let material = Lambertian::new(color!(0.4, 0.2, 0.1));
    world.add(Arc::new(Sphere::new(point!(-4.0, 1.0, 0.0), 1.0, material)));

    let material = Metal::new(color!(0.7, 0.6, 0.5), 0.0);
    world.add(Arc::new(Sphere::new(point!(4.0, 1.0, 0.0), 1.0, material)));

    world
}

pub fn final_first() -> HittableList {
    let mut world = HittableList::new();
    let material_ground = Lambertian::new(color!(0.5, 0.5, 0.5));
    world.add(Arc::new(Sphere::new(
        point!(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>(0.0..=1.0);
            let center = point!(
                a as f64 + 0.9 * random(0.0..=1.0),
                0.2,
                b as f64 + 0.9 * random(0.0..=1.0)
            );
            if (center - point!(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::delimited(0.5..=1.0);
                    let fuzz = random(0.0..=0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material = Dielectric::new(1.5);
    world.add(Arc::new(Sphere::new(point!(0.0, 1.0, 0.0), 1.0, material)));

    let material = Lambertian::new(color!(0.4, 0.2, 0.1));
    world.add(Arc::new(Sphere::new(point!(-4.0, 1.0, 0.0), 1.0, material)));

    let material = Metal::new(color!(0.7, 0.6, 0.5), 0.0);
    world.add(Arc::new(Sphere::new(point!(4.0, 1.0, 0.0), 1.0, material)));

    world
}

pub fn two_sphere() -> HittableList {
    let mut world = HittableList::new();
    let checker = Box::new(CheckerTexture::from_color(
        0.32,
        color!(0.2, 0.3, 0.1),
        color!(0.9, 0.9, 0.9),
    ));
    let checker2 = Box::new(CheckerTexture::from_color(
        0.32,
        color!(0.2, 0.3, 0.1),
        color!(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new(
        point!(0.0, -10.0, 0.0),
        10.0,
        Lambertian::textured(checker),
    )));

    world.add(Arc::new(Sphere::new(
        point!(0.0, 10.0, 0.0),
        10.0,
        Lambertian::textured(checker2),
    )));
    world
}
