use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::renderer::{Renderer, SceneDetails};
use crate::screen::Screen;
use crate::types::{Color, P3, V3};
use crate::utility::{
    linear_to_gamma, make_prng_from, random_double, sample_square, sample_unit_disc,
};
use glam::DVec3;
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


    pub fn render_pixel(&self, i: usize, j: usize, scene: &dyn Hittable, renderer: &dyn Renderer) -> Color {
        let mut prng = make_prng_from(((i + 1) * (j + 1)) as u64);
        let scene_details = SceneDetails {
            background: self.background,
        };
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

        for _ in 0..self.samples_per_pixel {
            let r = self.get_ray(i, j, &mut prng);
            let mut in_flight = renderer.ray_color(&r, self.max_depth, scene, &scene_details, &mut prng);

            if !in_flight.x.is_finite() {
                in_flight.x = 0.0;
            }

            if !in_flight.y.is_finite() {
                in_flight.y = 0.0;
            }

            if !in_flight.z.is_finite() {
                in_flight.z = 0.0;
            }

            pixel_color += in_flight;
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

    pub fn render(&self, scene: &dyn Hittable, renderer :&dyn Renderer) {
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
            .map(|(i, j)| self.render_pixel(i, j, scene, renderer))
            .collect();

        let path = "output.png";
        screen.write_png(path);
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

pub fn initialize_camera(
    cam_params: (
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
    ),
) -> Camera {
    let (
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
    ) = cam_params;

    let aspect_ratio = image_width as f64 / image_height as f64;

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
