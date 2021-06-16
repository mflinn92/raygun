use raygun::write_ppm;

fn main() {
    if let Err(e) = write_ppm("test.ppm") {
        eprintln!("{}", e);
    };
}
