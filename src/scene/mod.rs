use crate::hit::{Hit, Hittable};
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::moving_sphere::MovingSphere;
use crate::sphere::Sphere;
use crate::utils::random;
use crate::Color;
use std::sync::Arc;

pub fn random_scene() -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = vec![];
    let material_ground = Arc::new(Lambertian::new(color!(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
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
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + vec3!(0.0, random(0.0..=0.5), 0.0);
                    world.push(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        0.1,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::delimited(0.5..=1.0);
                    let fuzz = random(0.0..=0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }

    let material = Arc::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(
        point!(0.0, 1.0, 0.0),
        1.0,
        material.clone(),
    )));

    let material = Arc::new(Lambertian::new(color!(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(
        point!(-4.0, 1.0, 0.0),
        1.0,
        material.clone(),
    )));

    let material = Arc::new(Metal::new(color!(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(
        point!(4.0, 1.0, 0.0),
        1.0,
        material.clone(),
    )));

    world
}

pub fn final_first() -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = vec![];
    let material_ground = Arc::new(Lambertian::new(color!(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
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
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::delimited(0.5..=1.0);
                    let fuzz = random(0.0..=0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }

    let material = Arc::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(
        point!(0.0, 1.0, 0.0),
        1.0,
        material.clone(),
    )));

    let material = Arc::new(Lambertian::new(color!(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(
        point!(-4.0, 1.0, 0.0),
        1.0,
        material.clone(),
    )));

    let material = Arc::new(Metal::new(color!(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(
        point!(4.0, 1.0, 0.0),
        1.0,
        material.clone(),
    )));

    world
}
