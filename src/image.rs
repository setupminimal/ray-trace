use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

use linya::Progress;
use rayon::prelude::*;

use crate::vec::*;

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Image {
    pub fn new<F>(width: usize, height: usize, f: F) -> Image
    where
        F: (Fn(usize, usize) -> Color) + Sync,
    {
        let mut locs = Vec::new();
        for j in 0..height {
            for i in 0..width {
                locs.push((i, height - j));
            }
        }

        let progress = Mutex::new(Progress::new());
        let bar = progress
            .lock()
            .unwrap()
            .bar((height * width).into(), "Rendering");

        let data = locs
            .par_iter()
            .map_with(0, |c, i| {
                let ind = 200;
                *c += 1;
                if *c % ind == 0 {
                    progress.lock().unwrap().inc_and_draw(&bar, 10);
                }
                i
            })
            .map(|(x, y)| f(*x, *y))
            .collect();

        Image {
            width,
            height,
            data,
        }
    }

    pub fn write(&self, loc: &str) -> Result<(), Box<dyn Error>> {
        let mut output = File::create(loc)?;
        let max_pixel = 255.99;

        output.write_all(b"P3\n")?;
        output.write_all(format!("{} {}\n", self.width, self.height).as_bytes())?;
        output.write_all(b"255\n")?;

        for pix in &self.data {
            // TODO clamp to actual maximum

            // Note: gamma=2.0
            let r = (pix.x.sqrt() * max_pixel) as u16;
            let g = (pix.y.sqrt() * max_pixel) as u16;
            let b = (pix.z.sqrt() * max_pixel) as u16;
            output.write_all(format!("{} {} {} ", r, g, b).as_bytes())?;
        }

        Ok(())
    }
}
