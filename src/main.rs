extern crate clap;
extern crate rand;

use std::error::Error;
use std::fs::File;

use clap::{App, Arg};

mod image;
mod integrator;
mod scene;
mod vec;

mod parse;
use parse::parse_file;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Raytrace")
        .version("2.0")
        .author("Daroc Alden <setupminimal@gmail.com>")
        .about("Simple single-file raytracer")
        .arg(
            Arg::with_name("output-file")
                .short("o")
                .long("output-file")
                .default_value("out.ppm")
                .takes_value(true)
                .help("Output file name"),
        )
        .arg(
            Arg::with_name("input-file")
                .short("f")
                .long("input-file")
                .takes_value(true)
                .help("Input file in pbrtv3 format"),
        )
        .get_matches();

    let output_file = matches.value_of("output-file").unwrap();
    let input_file = matches.value_of("input-file").unwrap();

    let (scene, mut integrator) = parse_file(input_file)?;

    let image = integrator.render();

    image.write_to(output_file)?;

    Ok(())
}
