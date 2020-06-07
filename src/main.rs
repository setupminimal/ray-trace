#[macro_use] extern crate impl_ops;
extern crate rand;
extern crate clap;

use std::error::Error;
use std::fs::File;
use std::io::Write;

use rayon::prelude::*;

use clap::{Arg, App};

mod vec;
use crate::vec::*;

mod material;
use crate::material::Material::*;

mod sample;
use crate::sample::*;

mod shape;
use crate::shape::*;

mod camera;
use crate::camera::*;


fn random_scene() -> HitableGroup {
    let mut things : Vec<Sphere> = Vec::new();
    let mut planes : Vec<Plane> = Vec::new();

    planes.push(Plane {
        origin: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        material: Lambertian(Vec3::new(0.5, 0.6, 0.75)),
    });

    planes.push(Plane {
        origin: Vec3::new(0.0, 0.0, 100.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
        material: NoScatter(Vec3::new(1.0, 1.0, 1.0))
    });

    things.push(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric(Vec3::new(1.0, 1.0, 1.0), 1.5),
    });

    things.push(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian(Vec3::new(0.4, 0.2, 0.1)),
    });

    things.push(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Vec3::new(a as Float + 0.9 * random(), 0.2, b as Float + 0.9 * random());
            let offset = Vec3::new(4.0, 0.2, 0.0);

            if (&center - offset).norm() > 0.9 {
                things.push(Sphere {
                    center,
                    radius: 0.2,
                    material: if choose_mat < 0.6 {
                        Lambertian(Vec3::new(random() * random(),
                                             random() * random(),
                                             random() * random()))
                    } else if choose_mat < 0.9 {
                        Metal(Vec3::new(random() * 0.5 + 0.5,
                                        random() * 0.5 + 0.5,
                                        random() * 0.5 + 0.5),
                              random())
                    } else {
                        Dielectric(Vec3::new(1.0, 1.0, 1.0), 1.5)
                    },
                });
            }
        }
    }

    HitableGroup { spheres: things, planes }
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO use specific random seed

    let matches =
        App::new("Raytrace")
        .version("1.0")
        .author("Daroc Alden <setupminimal@gmail.com>")
        .about("Simple single-file raytracer")
        .arg(Arg::with_name("samples")
             .short("s")
             .long("samples")
             .default_value("50")
             .help("Number of rays to sample per pixel.")
             .takes_value(true))
        .arg(Arg::with_name("width")
             .short("x")
             .long("width")
             .default_value("600")
             .help("Width of generated image.")
             .takes_value(true))
        .arg(Arg::with_name("height")
             .short("y")
             .long("height")
             .default_value("400")
             .help("Height of generated image.")
             .takes_value(true))
        .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .default_value("out.ppm")
             .takes_value(true)
             .help("Output file name"))
        .arg(Arg::with_name("fov")
             .long("fov")
             .default_value("40.0")
             .help("Vertical field of view.")
             .takes_value(true))
        .arg(Arg::with_name("aperature")
             .short("a")
             .long("aperature")
             .default_value("0.07")
             .help("Lens aperature.")
             .takes_value(true))
        .arg(Arg::with_name("parallel")
             .short("p")
             .default_value("true")
             .help("Use parallel code.")
             .takes_value(true))
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

    let lookfrom = Vec3::new(9.0, 2.0, 5.0);
    let lookat = Vec3::new(0.0, 1.0, 0.0);
    let cam = Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0),
                          nx, ny, fov, aperature, samples);
    let world = random_scene();

    let lines = (0..ny).collect::<Vec<_>>();

    let process_line = |j| {
        let rj = ny - 1 - j;
        let mut v = vec!();
        for i in 0..nx {
            let pix = cam.point(i, rj, &world).as_ppm();
            v.push(pix.as_bytes().to_vec());
        }
        v
    };

    for res in if parallel {
        lines.par_iter().map(process_line).collect::<Vec<_>>()
    } else {
        lines.iter().map(process_line).collect::<Vec<_>>()
    }
    {
        for pix in res {
            output.write_all(pix.as_ref())?;
        }
    }

    Ok(())
}
