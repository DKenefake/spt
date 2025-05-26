use crate::perlin::Perlin;
use crate::types::{Color, P3};
use image::ImageReader;
use image::RgbaImage;
use std::path::Path;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &P3) -> Color;
}

pub struct SolidColor {
    pub albedo: Color,
}

impl SolidColor {
    pub const fn new() -> Self {
        Self {
            albedo: Color::new(0.5, 0.5, 0.5),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &P3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    pub inv_scale: f64,
    pub even: Arc<dyn Texture>,
    pub odd: Arc<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &P3) -> Color {
        let xyz = (p * self.inv_scale).floor().as_i64vec3();
        let is_even = (xyz.x + xyz.y + xyz.z) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub const fn from(noise: Perlin, scale: f64) -> Self {
        Self { noise, scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &P3) -> Color {
        Color::ONE * self.noise.noise(&(self.scale * p))
    }
}

struct ImageTexture {
    data: Option<RgbaImage>,
}

impl ImageTexture {
    pub fn new(file_path: &str) -> Self {
        let data = ImageReader::open(Path::new(file_path))
            .ok()
            .and_then(|x| x.decode().map(|x| x.to_rgba8()).ok());
        Self { data }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &P3) -> Color {
        if let Some(data) = &self.data {
            let u = u.clamp(0.0, 1.0);
            let v = 1.0 - v.clamp(0.0, 1.0);

            let (i, j) = {
                let mut i = (u * data.width() as f64) as u32;
                let mut j = (v * data.width() as f64) as u32;

                if i >= data.width() {
                    i = data.width() - 1;
                }

                if j >= data.height() {
                    j = data.height() - 1;
                }

                (i, j)
            };

            let color_scale = 1.0 / 255.0;
            let pixel = data.get_pixel(i, j).0;

            Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64) * color_scale
        } else {
            Color::new(0.0, 1.0, 1.0)
        }
    }
}
