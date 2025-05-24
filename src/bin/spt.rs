use std::rc::Rc;
use spt::camera::initialize_camera;
use spt::types::{Color, P3};
use spt::hittable_list::HittableList;
use spt::sphere::Sphere;
use spt::lambertian::{Dielectric, Lambertian, Metal};




fn main() {

    // World
    let mut world = HittableList::new();

    let mat_ground = Rc::new(Lambertian{albedo: Color::new(0.8, 0.8, 0.0)});
    let mat_center = Rc::new(Lambertian{albedo: Color::new(0.1, 0.2, 0.5)});
    let mat_left = Rc::new(Dielectric{refraction_index: 1.0 / 1.33});
    let mat_right = Rc::new(Metal{albedo: Color::new(0.8, 0.6, 0.2), fuzz: 1.0});

    world.add(Box::new(Sphere::from(P3::new(0.0, -100.5, -1.0), 100.0, mat_ground)));
    world.add(Box::new(Sphere::from(P3::new(0.0, 0.0, -1.2), 0.5, mat_center)));
    world.add(Box::new(Sphere::from(P3::new(-1.0, 0.0, -1.0), 0.5, mat_left)));
    world.add(Box::new(Sphere::from(P3::new(1.0, 0.0, -1.0), 0.5, mat_right)));


    let camera = initialize_camera();

    camera.render(&world);

}