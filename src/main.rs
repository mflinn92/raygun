use raygun::{render, Ray, Vec3, HEIGHT, WIDTH};
use std::fs::File;

fn main() {
    let img_file = File::create("test.ppm").unwrap();

    let r = render(img_file, |mut img_writer| {
        img_writer.header(WIDTH, HEIGHT)?;

        let lower_left = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);

        for j in (0..HEIGHT).rev() {
            for i in 0..WIDTH {
                let u = i as f64 / WIDTH as f64;
                let v = j as f64 / HEIGHT as f64;
                let direction = lower_left + u * horizontal + v * vertical;
                let r = Ray::new(origin, direction);
                let color = r.color().unwrap();
                let point: String = (color * 255.99).into();
                img_writer.append(point.as_str())?;
            }
        }
        Ok(())
    });

    if let Err(e) = r {
        eprintln!("{}", e)
    }
}
