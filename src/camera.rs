use crate::bvh::BVHNode;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::screen::Screen;
use crate::types::{Color, P3, V3};
use crate::utility::{
    linear_to_gamma, make_prng_from, random_double, sample_square, sample_unit_disc,
};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use smolprng::{JsfLarge, PRNG};

pub struct Camera {
    image_height: usize,
    image_width: usize,
    samples_per_pixel: usize,
    fov: f64,
    defocus_angle: f64,
    focus_dist: f64,
    max_depth: usize,
    camera_center: P3,
    pixel00_loc: P3, //location of pixel 0,0
    pixel_delta_u: V3,
    pixel_delta_v: V3,
    u: V3,
    v: V3,
    w: V3,
    defocus_disk_u: V3,
    defocus_disk_v: V3,
    background: Color,
}

impl Camera {
    pub fn ray_color(
        &self,
        r: &Ray,
        depth: usize,
        world: &BVHNode,
        prng: &mut PRNG<JsfLarge>,
    ) -> Color {
        if depth == 0 {
            return Color::ZERO;
        }

        let hit_rec = world.hit(r, &Interval::from(0.001, f64::MAX), prng);

        if hit_rec.is_none() {
            return self.background;
        }

        let rec = hit_rec.unwrap();

        let color_from_emission = rec.material.emitted(rec.u, rec.v, &rec.p);

        let scatter_attempt = rec.material.scatter(r, &rec, prng);

        if scatter_attempt.is_none() {
            return color_from_emission;
        }

        match scatter_attempt {
            None => color_from_emission,
            Some((sray, scolor)) => {
                let color_from_scatter = scolor * self.ray_color(&sray, depth - 1, world, prng);
                color_from_scatter + color_from_emission
            }
        }
    }

    pub fn render_pixel(&self, i: usize, j: usize, scene: &BVHNode) -> Color {
        let mut prng = make_prng_from(((i + 1) * (j + 1)) as u64);

        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

        for _ in 0..self.samples_per_pixel {
            let r = self.get_ray(i, j, &mut prng);
            pixel_color += Self::ray_color(self, &r, self.max_depth, scene, &mut prng);
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

    pub fn render(&self, scene: &BVHNode) {
        let mut screen = Screen::from(self.image_width, self.image_height);

        let mut pixel_locs = Vec::new();

        //Render
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                pixel_locs.push((i, j));
            }
        }

        screen.screen_data = pixel_locs
            .into_par_iter()
            .map(|(i, j)| self.render_pixel(i, j, scene))
            .collect();

        let path = "output.ppm";
        screen.write(path);
    }

    pub fn get_ray(&self, i: usize, j: usize, prng: &mut PRNG<JsfLarge>) -> Ray {
        let offset = sample_square(prng);
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample(prng)
        };
        let ray_direction = pixel_sample - ray_origin;

        let ray_time = random_double(prng);

        Ray::from(&ray_origin, &ray_direction, ray_time)
    }

    pub fn defocus_disk_sample(&self, prng: &mut PRNG<JsfLarge>) -> V3 {
        let p = sample_unit_disc(prng);
        self.camera_center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}

pub fn initialize_camera() -> Camera {
    let image_width = 1200;
    let image_height = 500;
    let aspect_ratio = image_width as f64 / image_height as f64;
    let samples_per_pixel = 1000;
    let max_depth = 25;
    let fov = 20.0f64;

    // camera point set up
    let look_from = P3::new(13.0, 2.0, 3.0);
    let look_at = P3::new(0.0, 0.0, 0.0);
    let v_up = V3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.6;
    let focus_dist = 10.4;

    // Camera
    let theta: f64 = fov.to_radians();
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h * focus_dist;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_center = look_from;

    // calculate u, v, w
    let w = (look_from - look_at).normalize();
    let u = v_up.cross(w).normalize();
    let v = w.cross(u).normalize();

    // calculate the vectors across and down
    let viewport_u = viewport_width * u;
    let viewport_v = -viewport_height * v;

    // calculate the horizontal and vertical delta vector from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // calculate the location of the upper left pixel
    let viewport_upper_left = camera_center - focus_dist * w - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let defocus_radius = (defocus_angle / 2.0f64).to_radians().tan() * focus_dist;
    let defocus_disk_u = u * defocus_radius;
    let defocus_disk_v = v * defocus_radius;

    let background = Color::new(0.5, 0.5, 0.5);

    Camera {
        image_height,
        image_width,
        samples_per_pixel,
        fov,
        defocus_angle,
        focus_dist,
        max_depth,
        camera_center,
        pixel00_loc,
        pixel_delta_u,
        pixel_delta_v,
        u,
        v,
        w,
        defocus_disk_u,
        defocus_disk_v,
        background,
    }
}
