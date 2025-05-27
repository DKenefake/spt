#![warn(clippy::all, clippy::cargo, clippy::nursery, clippy::perf)]
#[allow(clippy::unused)]
pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod hit_record;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod lambertian;
pub mod material;
pub mod medium;
pub mod onb;
pub mod pdf;
pub mod perlin;
pub mod quad;
pub mod ray;
pub mod rotate;
pub mod screen;
pub mod sphere;
pub mod texture;
pub mod translate;
pub mod types;
pub mod utility;
