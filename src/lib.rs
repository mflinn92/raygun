use std::fs::File;

mod ppm;
mod vec;
use ppm::{Generator, Ppm};

pub const WIDTH: u32 = 200;
pub const HEIGHT: u32 = 100;

pub fn render<F>(path: &str, generator: F) -> std::io::Result<()>
where
    F: Fn(Box<dyn Generator>) -> std::io::Result<()>,
{
    let img = File::create(path)?;
    let ppm = Ppm::new(img);

    generator(Box::new(ppm))
}
