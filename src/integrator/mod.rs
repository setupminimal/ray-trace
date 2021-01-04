use crate::image::Image;

pub trait Integrator {
    fn render(&mut self) -> Image;
}
