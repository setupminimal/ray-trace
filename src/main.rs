#![allow(dead_code)]

use std::error::Error;

mod camera;
mod image;
mod material;
mod scene;
mod scenes;
mod shapes;
mod vec;
use crate::camera::Camera;
use crate::image::Image;
use crate::scene::Scene;
use crate::vec::*;

fn main() -> Result<(), Box<dyn Error>> {
    let output_file = "out.ppm";
    let width = 1200;
    let height = 800;

    let camera = Camera::new(
        20.0,
        width as f32 / height as f32,
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    let scene = scenes::random_scene();

    let image = Image::new(width, height, |x, y| {
        let rays = camera.ray(x, y, width, height);
        // TODO configurable max depth
        // TODO does using median or throwing out outliers make the color smoother?
        rays.iter().map(|ray| scene.color(ray, 50)).sum::<Color>() / rays.len() as f32
    });

    image.write(output_file)?;

    Ok(())
}
