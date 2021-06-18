use std::io::Write;

mod ppm;
mod vec;
use ppm::{Generator, Ppm};

pub const WIDTH: u32 = 200;
pub const HEIGHT: u32 = 100;

pub fn render<W, F>(writer: W, generator: F) -> std::io::Result<()>
where
    W: Write + 'static,
    F: Fn(Box<dyn Generator>) -> std::io::Result<()>,
{
    let ppm = Ppm::new(writer);

    generator(Box::new(ppm))
}
