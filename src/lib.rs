mod ppm;
mod ray;
mod vec;
pub use ppm::render;
pub use ray::Ray;
pub use vec::Vec3;
pub const WIDTH: u32 = 400;
pub const HEIGHT: u32 = 300;
