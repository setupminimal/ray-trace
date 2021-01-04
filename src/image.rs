use crate::vec::Vec3;

use std::error::Error;

pub struct Image {
    pixels: Vec<Vec<Vec3>>,
}

impl Image {
    pub fn write_to(&self, file: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
