use raygun::render;

fn main() {
    if let Err(e) = render("test.ppm") {
        eprintln!("{}", e);
    };
}
