use smolprng::{JsfLarge, PRNG};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::ScatterRay;
use crate::ray::Ray;
use crate::types::Color;

pub struct SceneDetails{
    pub background: Color,
}

pub trait Renderer: Sync + Send {

    fn ray_color(
        &self,
        r: &Ray,
        depth: usize,
        scene: &dyn Hittable,
        scene_details: &SceneDetails,
        prng: &mut PRNG<JsfLarge>,
    ) -> Color;

}

// just gets the normals and returns a color based on them
pub struct NormalRenderer{}

impl Renderer for NormalRenderer {
    fn ray_color(
        &self,
        r: &Ray,
        _depth: usize,
        scene: &dyn Hittable,
        _scene_details: &SceneDetails,
        prng: &mut PRNG<JsfLarge>,
    ) -> Color {


        if let Some(hit_record) = scene.hit(r, &Interval::casting_default(), prng) {

            return hit_record.normal * 0.5 + Color::new(0.5, 0.5, 0.5); // Simple shading
        }

        Color::ZERO
    }
}


pub struct FullRenderer {}

impl Renderer for FullRenderer {

    fn ray_color(
        &self,
        r: &Ray,
        depth: usize,
        world: &dyn Hittable,
        scene_details: &SceneDetails,
        prng: &mut PRNG<JsfLarge>,
    ) -> Color {

        if depth == 0 {
            return Color::ZERO;
        }

        let hit_rec = world.hit(r, &Interval::from(0.001, f64::MAX), prng);

        if hit_rec.is_none() {
            return scene_details.background;
        }

        let rec = hit_rec.unwrap();

        let color_from_emission = rec.material.emitted(r, &rec, rec.u, rec.v, &rec.p);

        let scatter_attempt = rec.material.scatter(r, &rec, prng);

        match scatter_attempt {
            None => color_from_emission,
            Some(scatter) => match scatter {
                ScatterRay::Specular {
                    specular_ray,
                    attenuation,
                } => attenuation * self.ray_color(&specular_ray, depth - 1, world, scene_details, prng),
                ScatterRay::Scatter { pdf, attenuation } => {
                    let scattered = Ray::from(&rec.p, &pdf.generate(r.time, prng), r.time);
                    let pdf_value = pdf.value(&scattered.direction, r.time, prng);

                    let sample_color = self.ray_color(&scattered, depth - 1, world, scene_details, prng);
                    let color_from_scatter = attenuation * pdf_value * sample_color / pdf_value;
                    color_from_emission + color_from_scatter
                }
            },
        }
    }
}