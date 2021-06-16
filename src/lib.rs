use std::fs::File;
use std::io::prelude::*;

pub fn write_ppm(path: &str) -> std::io::Result<()> {
    let mut img = File::create(path)?;
    let nx = 200;
    let ny = 100;

    write!(img, "P3\n{} {}\n255\n", nx, ny)?;

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2;
            let ir = (r * 255.99) as u32;
            let ig = (g * 255.99) as u32;
            let ib = (b * 255.99) as u32;
            writeln!(img, "{} {} {}", ir, ig, ib)?;
        }
    }
    Ok(())
}
