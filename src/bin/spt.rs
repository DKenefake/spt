use glam::DVec3;
use spt::bvh::BVHNode;
use spt::camera::initialize_camera;
use spt::hittable_list::HittableList;
use spt::lambertian::{Dielectric, DiffuseLight, Lambertian, Metal};
use spt::quad::Quad;
use spt::ray::Ray;
use spt::sphere::Sphere;
use spt::texture::{CheckerTexture, SolidColor};
use spt::types::{Color, P3, V3};
use spt::utility::{make_prng_default, random_double, random_double_in_range};
use std::sync::Arc;
use std::time::Instant;

fn quad_scene() {
    fn camera_set_up() -> (
        usize,
        usize,
        usize,
        usize,
        f64,
        DVec3,
        DVec3,
        DVec3,
        f64,
        f64,
        DVec3,
    ) {
        let image_width = 400;
        let image_height = 400;
        let samples_per_pixel = 1000;
        let max_depth = 50;
        let fov = 80.0f64;
        let look_from = P3::new(0.0, 0.0, 9.0);
        let look_at = P3::new(0.0, 0.0, 0.0);
        let v_up = V3::new(0.0, 1.0, 0.0);

        let defocus_angle = 0.0;
        let focus_dist = 10.4;
        let background = Color::new(0.8, 0.8, 0.8);

        (
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            fov,
            look_from,
            look_at,
            v_up,
            defocus_angle,
            focus_dist,
            background,
        )
    }

    let mut world = HittableList::new();

    let left_red = Arc::new(Lambertian::from_color(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::from_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::from_color(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::from_color(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::from_color(Color::new(0.2, 0.8, 0.8)));

    world.add(Box::new(Quad::new(
        P3::new(-3.0, -2.0, 5.0),
        -4.0 * V3::Z,
        4.0 * V3::Y,
        left_red,
    )));
    world.add(Box::new(Quad::new(
        P3::new(-2.0, -2.0, 0.0),
        4.0 * V3::X,
        4.0 * V3::Y,
        back_green,
    )));
    world.add(Box::new(Quad::new(
        P3::new(3.0, -2.0, 1.0),
        4.0 * V3::Z,
        4.0 * V3::Y,
        right_blue,
    )));
    world.add(Box::new(Quad::new(
        P3::new(-2.0, 3.0, 1.0),
        4.0 * V3::X,
        4.0 * V3::Z,
        upper_orange,
    )));
    world.add(Box::new(Quad::new(
        P3::new(-2.0, -3.0, 5.0),
        4.0 * V3::X,
        -4.0 * V3::Z,
        lower_teal,
    )));

    let world_bvh = BVHNode::from(&mut world.objects);

    let camera = initialize_camera(camera_set_up());

    camera.render(&world_bvh);
}

fn bouncing_balls() {
    fn camera_set_up() -> (
        usize,
        usize,
        usize,
        usize,
        f64,
        DVec3,
        DVec3,
        DVec3,
        f64,
        f64,
        DVec3,
    ) {
        let image_width = 1200;
        let image_height = 500;
        let samples_per_pixel = 150;
        let max_depth = 15;
        let fov = 20.0f64;

        let look_from = P3::new(13.0, 2.0, 3.0);
        let look_at = P3::new(0.0, 0.0, 0.0);
        let v_up = V3::new(0.0, 1.0, 0.0);

        let defocus_angle = 0.0;
        let focus_dist = 10.0;
        let background = Color::new(0.8, 0.8, 0.8);

        (
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            fov,
            look_from,
            look_at,
            v_up,
            defocus_angle,
            focus_dist,
            background,
        )
    }

    let mut world = HittableList::new();

    let checker = Lambertian::from_texture(Arc::new(CheckerTexture {
        inv_scale: 0.32,
        even: Arc::new(SolidColor {
            albedo: Color::new(0.2, 0.3, 0.1),
        }),
        odd: Arc::new(SolidColor {
            albedo: Color::new(0.9, 0.9, 0.9),
        }),
    }));

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
                    let center_2 = center + V3::new(0.0, 0.0 * random_double(&mut prng), 0.0);
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

    let camera = initialize_camera(camera_set_up());

    camera.render(&world_bvh);
}

fn still_balls() {
    fn camera_set_up() -> (
        usize,
        usize,
        usize,
        usize,
        f64,
        DVec3,
        DVec3,
        DVec3,
        f64,
        f64,
        DVec3,
    ) {
        let image_width = 2560;
        let image_height = 1440;
        let samples_per_pixel = 2560;
        let max_depth = 100;
        let fov = 20.0f64;

        let look_from = P3::new(13.0, 2.0, 3.0);
        let look_at = P3::new(0.0, 0.0, 0.0);
        let v_up = V3::new(0.0, 1.0, 0.0);

        let defocus_angle = 0.0;
        let focus_dist = 10.0;
        let background = Color::new(0.8, 0.8, 0.8);

        (
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            fov,
            look_from,
            look_at,
            v_up,
            defocus_angle,
            focus_dist,
            background,
        )
    }

    let mut world = HittableList::new();

    let checker = Lambertian::from_texture(Arc::new(CheckerTexture {
        inv_scale: 0.32,
        even: Arc::new(SolidColor {
            albedo: Color::new(0.2, 0.3, 0.1),
        }),
        odd: Arc::new(SolidColor {
            albedo: Color::new(0.9, 0.9, 0.9),
        }),
    }));

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
                    world.add(Box::new(Sphere::static_sphere(center, 0.2, sphere_mat)))
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

    let light_mat = Arc::new(DiffuseLight::from_color(Color::ONE * 5.0));

    let light_sphere = Box::new(Sphere::static_sphere(5.0 * V3::Y, 1.0, light_mat));

    world.add(light_sphere);

    let world_bvh = BVHNode::from(&mut world.objects);

    let mut world_2 = HittableList::new();

    world_2.add(Box::new(Sphere::static_sphere(
        P3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(checker),
    )));

    world_2.add(Box::new(world_bvh));

    let camera = initialize_camera(camera_set_up());

    camera.render(&world_2);
}

fn main() {
    let now = Instant::now();

    still_balls();

    let stop = Instant::now();

    let delta = (stop - now).as_secs_f64();

    println!("{} secs", delta);
    println!("Done Running!");
}
