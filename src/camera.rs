use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::screen::Screen;
use crate::types::{Color, P3, V3};
use crate::utility::{linear_to_gamma, make_prng_from, sample_square};
use smolprng::{JsfLarge, PRNG};

pub struct Camera {
    image_height: usize,
    image_width: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    camera_center: P3,
    pixel00_loc: P3, //location of pixel 0,0
    pixel_delta_u: V3,
    pixel_delta_v: V3,
}

impl Camera {
    pub fn ray_color(
        r: &Ray,
        depth: usize,
        world: &HittableList,
        prng: &mut PRNG<JsfLarge>,
    ) -> Color {
        if depth == 0 {
            return Color::ZERO;
        }

        match world.hit(r, &Interval::from(0.001, f64::MAX)) {
            None => {}
            Some(hr) => {
                let (is_so, scattered, attenuation) = hr.material.scatter(r, &hr, prng);
                return if is_so {
                    attenuation * Self::ray_color(&scattered, depth - 1, world, prng)
                } else {
                    Color::ZERO
                };
            }
        }

        let unit_dir = r.direction.normalize();
        let a = 0.5f64 * (unit_dir.y + 1.0);
        (1.0 - a) * Color::ONE + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render_pixel(&self, i: usize, j: usize, scene: &HittableList) -> Color {
        let mut prng = make_prng_from(((i + 1) * (j + 1)) as u64);

        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

        for _ in 0..self.samples_per_pixel {
            let r = self.get_ray(i, j, &mut prng);
            pixel_color += Self::ray_color(&r, self.max_depth, scene, &mut prng);
        }
        pixel_color /= self.samples_per_pixel as f64;

        pixel_color.x = linear_to_gamma(pixel_color.x);
        pixel_color.y = linear_to_gamma(pixel_color.y);
        pixel_color.z = linear_to_gamma(pixel_color.z);

        pixel_color = (255.99 * pixel_color)
            .floor()
            .clamp(Color::ZERO, Color::splat(255.0));

        pixel_color
    }

    pub fn render(&self, scene: &HittableList) {
        let mut screen = Screen::from(self.image_width, self.image_height);

        //Render
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_color = self.render_pixel(i, j, scene);
                screen.set(pixel_color, i, j);
            }
        }

        let path = "output.ppm";
        screen.write(path);
    }

    pub fn get_ray(&self, i: usize, j: usize, prng: &mut PRNG<JsfLarge>) -> Ray {
        let offset = sample_square(prng);
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;

        let ray_origin = self.camera_center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::from(&ray_origin, &ray_direction)
    }
}

pub fn initialize_camera() -> Camera {
    let image_width = 1280 / 2;
    let image_height = 720 / 2;
    let aspect_ratio = image_width as f64 / image_height as f64;
    let samples_per_pixel = 5000;
    let max_depth = 30;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_center = P3::ZERO;

    // calculate the vectors across and down
    let viewport_u = V3::new(viewport_width, 0.0, 0.0);
    let viewport_v = V3::new(0.0, -viewport_height, 0.0);

    // calculate the horizontal and vertical delta vector from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - V3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    Camera {
        image_height,
        image_width,
        samples_per_pixel,
        max_depth,
        camera_center,
        pixel00_loc,
        pixel_delta_u,
        pixel_delta_v,
    }
}
