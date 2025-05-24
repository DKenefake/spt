use spt::camera::initialize_camera;
use spt::hittable_list::HittableList;
use spt::lambertian::{Dielectric, Lambertian, Metal};
use spt::sphere::Sphere;
use spt::types::{Color, P3};
use std::sync::Arc;

fn main() {
    // World
    let mut world = HittableList::new();

    let mat_ground = Arc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let mat_center = Arc::new(Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let mat_left = Arc::new(Dielectric {
        refraction_index: 1.5,
    });
    let mat_bubble = Arc::new(Dielectric {
        refraction_index: 1.0 / 1.5,
    });

    let mat_right = Arc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.00,
    });

    world.add(Box::new(Sphere::from(
        P3::new(0.0, -100.5, -1.0),
        100.0,
        mat_ground,
    )));
    world.add(Box::new(Sphere::from(
        P3::new(0.0, 0.0, -1.2),
        0.5,
        mat_center,
    )));
    world.add(Box::new(Sphere::from(
        P3::new(-1.0, 0.0, -1.0),
        0.5,
        mat_left,
    )));
    world.add(Box::new(Sphere::from(
        P3::new(-1.0, 0.0, -1.0),
        0.4,
        mat_bubble,
    )));
    world.add(Box::new(Sphere::from(
        P3::new(1.0, 0.0, -1.0),
        0.5,
        mat_right,
    )));


    let camera = initialize_camera();

    camera.render(&world);
}
