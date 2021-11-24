extern crate clap;
extern crate rand;

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

use linya::Progress;

use rayon::prelude::*;

use clap::{App, Arg};

mod vec;
use crate::vec::*;

mod material;
mod sample;
mod shape;
use crate::shape::*;

mod camera;
use crate::camera::*;

mod scenes;
use crate::scenes::random_scene;

fn main() -> Result<(), Box<dyn Error>> {
    // TODO use specific random seed

    let matches = App::new("Raytrace")
        .version("1.0")
        .author("Daroc Alden <setupminimal@gmail.com>")
        .about("A raytracer from scratch")
        .arg(
            Arg::with_name("samples")
                .short("s")
                .long("samples")
                .default_value("50")
                .help("Number of rays to sample per pixel.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("width")
                .short("x")
                .long("width")
                .default_value("300")
                .help("Width of generated image.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("height")
                .short("y")
                .long("height")
                .default_value("200")
                .help("Height of generated image.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .default_value("out.ppm")
                .takes_value(true)
                .help("Output file name"),
        )
        .arg(
            Arg::with_name("fov")
                .long("fov")
                .default_value("20.0")
                .help("Vertical field of view.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("aperature")
                .short("a")
                .long("aperature")
                .default_value("0.1")
                .help("Lens aperature.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("parallel")
                .short("p")
                .default_value("true")
                .help("Use parallel code.")
                .takes_value(true),
        )
        .get_matches();

    let samples = matches.value_of("samples").unwrap().parse::<u16>()?;
    let nx = matches.value_of("width").unwrap().parse::<u16>()?;
    let ny = matches.value_of("height").unwrap().parse::<u16>()?;
    let file = matches.value_of("file").unwrap();
    let fov = matches.value_of("fov").unwrap().parse::<Float>()?;
    let aperature = matches.value_of("aperature").unwrap().parse::<Float>()?;
    let parallel = matches.value_of("parallel").unwrap().parse::<bool>()?;

    let mut output = File::create(file)?;
    output.write_all(b"P3\n")?;
    output.write_all(format!("{} {}\n", nx, ny).as_bytes())?;
    output.write_all(b"255\n")?;

    let lookfrom = Vec3::new(13.0, 1.5, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);

    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        nx,
        ny,
        fov,
        aperature,
        samples,
        10.0, // TODO make focus dist argument
    );
    let world = random_scene();

    let lines = (0..ny).collect::<Vec<_>>();

    let progress = Mutex::new(Progress::new());
    let bar = progress.lock().unwrap().bar(ny.into(), "Rendering");

    let process_line = |j| {
        let rj = ny - 1 - j;
        let mut v = vec![];
        for i in 0..nx {
            let pix = cam.point(i, rj, &world).as_ppm();
            v.push(pix.as_bytes().to_vec());
        }
        progress.lock().unwrap().inc_and_draw(&bar, 1);
        v
    };

    for res in if parallel {
        lines.par_iter().map(process_line).collect::<Vec<_>>()
    } else {
        lines.iter().map(process_line).collect::<Vec<_>>()
    } {
        for pix in res {
            output.write_all(pix.as_ref())?;
        }
    }

    Ok(())
}
