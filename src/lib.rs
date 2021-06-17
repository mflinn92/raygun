use std::fs::File;
mod ppm;
mod vec;
use ppm::Ppm;

const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;

pub fn render(path: &str) -> std::io::Result<()> {
    let img = File::create(path)?;
    let mut ppm = Ppm::new(img);

    ppm.header(WIDTH, HEIGHT)?;

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let r = ((i as f64 / WIDTH as f64) * 255.99) as u8;
            let g = ((j as f64 / HEIGHT as f64) * 255.99) as u8;
            let b = (0.2 * 255.99) as u8;
            let point = &format!("{} {} {}", r, g, b);
            ppm.append(point)?;
        }
    }
    Ok(())
}
