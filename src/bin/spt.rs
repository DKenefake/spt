use spt::camera::initialize_camera;
use spt::hittable_list::HittableList;
use spt::lambertian::{Dielectric, Lambertian, Metal};
use spt::sphere::Sphere;
use spt::types::{Color, P3};
use std::sync::Arc;
use spt::utility::{make_prng_default, random_double, random_double_in_range};

fn main() {
    // World
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian{albedo: Color::new(0.5, 0.5, 0.5)});
    world.add(Box::new(Sphere::from(P3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    let mut prng = make_prng_default();


    for a in -11..11{
        for b in -11..11{

            let choose_mat = random_double(& mut prng);
            let center = P3::new(a as f64 + 0.9*random_double(& mut prng), 0.2, b as f64 + 0.9*random_double(& mut prng));

            if (center - P3::new(4.0, 0.2, 0.0)).length() > 0.9{

                if choose_mat <= 0.8{
                    let r = random_double(& mut prng);
                    let g = random_double(& mut prng);
                    let b = random_double(& mut prng);
                    let albedo = Color::new(r*r, g*g, b*b);
                    let sphere_mat = Arc::new(Lambertian{albedo});
                    world.add(Box::new(Sphere::from(center, 0.2, sphere_mat)))
                } else if choose_mat <= 0.95 {
                    let r = random_double_in_range(& mut prng, 0.5, 1.0);
                    let g = random_double_in_range(& mut prng, 0.5, 1.0);
                    let b = random_double_in_range(& mut prng, 0.5, 1.0);
                    let albedo = Color::new(r, g, b);
                    let fuzz = random_double_in_range(& mut prng, 0.0, 0.5);
                    let sphere_mat = Arc::new(Metal{albedo, fuzz});
                    world.add(Box::new(Sphere::from(center, 0.2, sphere_mat)))
                } else{
                    let sphere_mat = Arc::new(Dielectric{refraction_index: 1.5});
                    world.add(Box::new(Sphere::from(center, 0.2, sphere_mat)))
                }

            }

        }
    }

    let mat_1 = Arc::new(Dielectric{refraction_index: 1.5});
    let mat_2 = Arc::new(Lambertian{albedo: Color::new(0.4, 0.2, 0.1)});
    let mat_3 = Arc::new(Metal{albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0});

    world.add(Box::new(Sphere::from(P3::new(0.0, 1.0, 0.0), 1.0, mat_1)));
    world.add(Box::new(Sphere::from(P3::new(-4.0, 1.0, 0.0), 1.0, mat_2)));
    world.add(Box::new(Sphere::from(P3::new(4.0, 1.0, 0.0), 1.0, mat_3)));


    // let mat_ground = Arc::new(Lambertian {
    //     albedo: Color::new(0.8, 0.8, 0.0),
    // });
    // let mat_center = Arc::new(Lambertian {
    //     albedo: Color::new(0.1, 0.2, 0.5),
    // });
    // let mat_left = Arc::new(Dielectric {
    //     refraction_index: 1.5,
    // });
    // let mat_bubble = Arc::new(Dielectric {
    //     refraction_index: 1.0 / 1.5,
    // });
    //
    // let mat_right = Arc::new(Metal {
    //     albedo: Color::new(0.8, 0.6, 0.2),
    //     fuzz: 1.00,
    // });
    //
    // world.add(Box::new(Sphere::from(
    //     P3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     mat_ground,
    // )));
    // world.add(Box::new(Sphere::from(
    //     P3::new(0.0, 0.0, -1.2),
    //     0.5,
    //     mat_center,
    // )));
    // world.add(Box::new(Sphere::from(
    //     P3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     mat_left,
    // )));
    // world.add(Box::new(Sphere::from(
    //     P3::new(-1.0, 0.0, -1.0),
    //     0.4,
    //     mat_bubble,
    // )));
    // world.add(Box::new(Sphere::from(
    //     P3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     mat_right,
    // )));

    let camera = initialize_camera();

    camera.render(&world);
}
