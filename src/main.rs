use raygun::{render, HEIGHT, WIDTH};
use std::fs::File;

fn main() {
    let img_file = File::create("test.ppm").unwrap();

    let r = render(img_file, |mut img_writer| {
        img_writer.header(WIDTH, HEIGHT)?;

        for j in (0..HEIGHT).rev() {
            for i in 0..WIDTH {
                let r = ((i as f64 / WIDTH as f64) * 255.99) as u8;
                let g = ((j as f64 / HEIGHT as f64) * 255.99) as u8;
                let b = (0.2 * 255.99) as u8;
                let point = &format!("{} {} {}", r, g, b);
                img_writer.append(point)?;
            }
        }
        Ok(())
    });

    if let Err(e) = r {
        eprintln!("{}", e)
    }
}
