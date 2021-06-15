use raygun::write_ppm;

fn main() {
    match write_ppm("test.ppm") {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}
