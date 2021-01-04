use std::error::Error;

use crate::integrator::Integrator;
use crate::scene::Scene;

pub fn parse_file(path: &str) -> Result<(Box<dyn Scene>, Box<dyn Integrator>), Box<dyn Error>> {
    todo!()
}
