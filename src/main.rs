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
use crate::material::Material::*;

mod sample;
use crate::sample::*;

mod shape;
use crate::shape::*;

mod camera;
use crate::camera::*;

#[allow(dead_code)]
fn pipedream(t: Float) -> HitableGroup {
    let mut planes: Vec<Plane> = Vec::new();
    let mut cylinders: Vec<Cylinder> = Vec::new();

    planes.push(Plane {
        origin: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        material: Lambertian(Vec3::new(0.2, 0.2, 0.2)),
    });

    planes.push(Plane {
        origin: Vec3::new(60.0 * 60.0 * 2.0, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        material: Lambertian(Vec3::new(0.2, 0.2, 0.2)),
    });

    cylinders.push(Cylinder {
        radius: 10.0,
        material: Sun(t),
    });

    cylinders.push(Cylinder {
        radius: 60.0 * 8.0,
        material: BiasedLambertian(Vec3::new(0.2, 0.7, 0.2)),
    });

    HitableGroup {
        planes,
        spheres: KDTree::new(Vec::new()),
        cylinders,
    }
}

#[allow(dead_code)]
fn plain_scene() -> HitableGroup {
    let mut things: Vec<Sphere> = Vec::new();
    let mut planes: Vec<Plane> = Vec::new();

    planes.push(Plane {
        origin: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        material: Metal(Vec3::new(0.5, 0.6, 0.75), 0.9),
    });

    things.push(Sphere {
        center: Vec3::new(-2.0, 0.0, 0.0),
        radius: 0.5,
        material: Metal(Vec3::new(0.1, 0.1, 0.1), 0.5),
    });

    HitableGroup {
        spheres: KDTree::new(things),
        planes,
        cylinders: Vec::new(),
    }
}

fn random_scene() -> HitableGroup {
    let mut things: Vec<Sphere> = Vec::new();
    let mut planes: Vec<Plane> = Vec::new();

    planes.push(Plane {
        origin: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        material: Lambertian(Vec3::new(0.70, 0.73, 0.75)),
    });

    things.push(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric(Vec3::new(1.0, 1.0, 1.0), 1.5),
    });

    things.push(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian(Vec3::new(0.6, 0.4, 0.1)),
    });

    things.push(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Vec3::new(
                a as Float + 0.9 * random(),
                0.2,
                b as Float + 0.9 * random(),
            );
            let offset = Vec3::new(4.0, 0.2, 0.0);

            if (&center - offset).norm() > 0.9 {
                things.push(Sphere {
                    center,
                    radius: 0.2,
                    material: if choose_mat < 0.6 {
                        Lambertian(Vec3::new(
                            random() * random(),
                            random() * random(),
                            random() * random(),
                        ))
                    } else if choose_mat < 0.9 {
                        Metal(
                            Vec3::new(
                                random() * 0.5 + 0.5,
                                random() * 0.5 + 0.5,
                                random() * 0.5 + 0.5,
                            ),
                            random(),
                        )
                    } else {
                        Dielectric(Vec3::new(1.0, 1.0, 1.0), 1.5)
                    },
                });
            }
        }
    }

    let cylinders = Vec::new();

    HitableGroup {
        spheres: KDTree::new(things),
        planes,
        cylinders,
    }
}

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
                .default_value("40.0")
                .help("Vertical field of view.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("aperature")
                .short("a")
                .long("aperature")
                .default_value("0.0001")
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

    let lookfrom = Vec3::new(5.0, 2.0, 7.0);
    let lookat = Vec3::new(0.0, 1.0, 0.0);

    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        nx,
        ny,
        fov,
        aperature,
        samples,
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
