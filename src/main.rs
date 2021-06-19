use raygun::{render, Vec3, HEIGHT, WIDTH};
use std::fs::File;

fn main() {
    let img_file = File::create("test.ppm").unwrap();

    let r = render(img_file, |mut img_writer| {
        img_writer.header(WIDTH, HEIGHT)?;

        for j in (0..HEIGHT).rev() {
            for i in 0..WIDTH {
                let vec = Vec3::new(i as f64 / WIDTH as f64, j as f64 / HEIGHT as f64, 0.2);
                let point = vec * 255.99;
                let point: String = point.into();
                img_writer.append(point.as_str())?;
            }
        }
        Ok(())
    });

    if let Err(e) = r {
        eprintln!("{}", e)
    }
}
