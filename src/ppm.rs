use std::io::prelude::*;

pub fn render<W, F>(writer: W, generator: F) -> std::io::Result<()>
where
    W: Write + 'static,
    F: Fn(BoxedGenerator) -> std::io::Result<()>,
{
    generator(Ppm::new(writer))
}

type BoxedGenerator = Box<dyn Generator>;

#[derive(Debug)]
pub struct Ppm<W: Write> {
    writer: W,
}

pub trait Generator {
    fn header(&mut self, width: u32, height: u32) -> std::io::Result<()>;
    fn append(&mut self, data: &str) -> std::io::Result<()>;
}

impl<W: Write> Ppm<W> {
    pub fn new(writer: W) -> Box<Self> {
        Box::new(Ppm { writer })
    }
}

impl<W: Write> Generator for Ppm<W> {
    fn header(&mut self, width: u32, height: u32) -> std::io::Result<()> {
        write!(self.writer, "P3\n{} {}\n255\n", width, height)
    }

    fn append(&mut self, point_str: &str) -> std::io::Result<()> {
        writeln!(self.writer, "{}", point_str)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn write_header() {
        let writer = Vec::new();
        let mut ppm = Ppm::new(writer);

        let width = 60;
        let height = 40;

        ppm.header(width, height).unwrap();
        let expected = format!("P3\n{} {}\n255\n", width, height);
        assert_eq!(expected, String::from_utf8(ppm.writer).unwrap());
    }

    #[test]
    fn ppm_append() {
        let writer = Vec::new();
        let mut ppm = Ppm::new(writer);

        let point = "1 23 120";
        ppm.append(point).unwrap();
        let expected = format!("{}\n", point);
        assert_eq!(expected, String::from_utf8(ppm.writer).unwrap());
    }

    #[test]
    fn ppm_write() {
        let writer = Vec::new();
        let mut ppm = Ppm::new(writer);

        let width = 80;
        let height = 60;

        ppm.header(width, height).unwrap();

        let points = vec!["1 2 3", "4 5 6", "7 8 9"];
        for point in points.iter() {
            ppm.append(point).unwrap();
        }

        let expected = format!(
            "P3\n{} {}\n255\n{}\n{}\n{}\n",
            width, height, points[0], points[1], points[2],
        );

        assert_eq!(expected, String::from_utf8(ppm.writer).unwrap());
    }
}
