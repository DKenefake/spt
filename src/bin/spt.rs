use spt::bvh::BVHNode;
use spt::camera::initialize_camera;
use spt::hittable_list::HittableList;
use spt::lambertian::{Dielectric, Lambertian, Metal};
use spt::ray::Ray;
use spt::sphere::Sphere;
use spt::types::{Color, P3, V3};
use spt::utility::{make_prng_default, random_double, random_double_in_range};
use std::sync::Arc;
use std::time::Instant;
use spt::texture::{CheckerTexture, SolidColor};

fn bounding_spheres() {
    // World
    let mut world = HittableList::new();

    let checker = Lambertian::from_texture(Arc::new(CheckerTexture{inv_scale: 0.32, even: Arc::new(SolidColor{albedo: Color::new(0.2, 0.3, 0.1)}), odd: Arc::new(SolidColor{albedo: Color::new(0.9, 0.9, 0.9)})}));

    world.add(Box::new(Sphere::static_sphere(
        P3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(checker),
    )));

    let mut prng = make_prng_default();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(&mut prng);
            let center = P3::new(
                a as f64 + 0.9 * random_double(&mut prng),
                0.2,
                b as f64 + 0.9 * random_double(&mut prng),
            );

            if (center - P3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat <= 0.8 {
                    let r = random_double(&mut prng);
                    let g = random_double(&mut prng);
                    let b = random_double(&mut prng);
                    let albedo = Color::new(r * r, g * g, b * b);
                    let sphere_mat = Arc::new(Lambertian::from_color(albedo));
                    let center_2 = center + V3::new(0.0, 0.5 * random_double(&mut prng), 0.0);
                    world.add(Box::new(Sphere::from(
                        Ray::from(&center, &(center_2 - center), 0.0),
                        0.2,
                        sphere_mat,
                    )))
                } else if choose_mat <= 0.95 {
                    let r = random_double_in_range(&mut prng, 0.5, 1.0);
                    let g = random_double_in_range(&mut prng, 0.5, 1.0);
                    let b = random_double_in_range(&mut prng, 0.5, 1.0);
                    let albedo = Color::new(r, g, b);
                    let fuzz = random_double_in_range(&mut prng, 0.0, 0.5);
                    let sphere_mat = Arc::new(Metal { albedo, fuzz });
                    world.add(Box::new(Sphere::static_sphere(center, 0.2, sphere_mat)))
                } else {
                    let sphere_mat = Arc::new(Dielectric {
                        refraction_index: 1.5,
                    });
                    world.add(Box::new(Sphere::static_sphere(center, 0.2, sphere_mat)))
                }
            }
        }
    }

    let mat_1 = Arc::new(Dielectric {
        refraction_index: 1.5,
    });
    let mat_2 = Arc::new(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));

    let mat_3 = Arc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });

    world.add(Box::new(Sphere::static_sphere(
        P3::new(0.0, 1.0, 0.0),
        1.0,
        mat_1,
    )));
    world.add(Box::new(Sphere::static_sphere(
        P3::new(-4.0, 1.0, 0.0),
        1.0,
        mat_2,
    )));
    world.add(Box::new(Sphere::static_sphere(
        P3::new(4.0, 1.0, 0.0),
        1.0,
        mat_3,
    )));

    let world_bvh = BVHNode::from(&mut world.objects);

    let camera = initialize_camera();

    let now = Instant::now();

    // 19.799021 secs
    // Done Running!

    camera.render(&world_bvh);

    let stop = Instant::now();

    let delta = (stop - now).as_secs_f64();

    println!("{} secs", delta);
    println!("Done Running!");
}

fn main() {}