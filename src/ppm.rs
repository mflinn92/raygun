use std::io::prelude::*;

#[derive(Debug)]
pub struct Ppm<W: Write> {
    writer: W,
}

impl<W: Write> Ppm<W> {
    pub fn new(writer: W) -> Self {
        Ppm { writer }
    }

    pub fn header(&mut self, width: u32, height: u32) -> std::io::Result<()> {
        write!(self.writer, "P3\n{} {}\n255\n", width, height)
    }

    pub fn append(&mut self, point_str: &str) -> std::io::Result<()> {
        writeln!(self.writer, "{}", point_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_header() {
        let writer = Vec::new();
        let mut ppm = Ppm::new(writer);

        let width = 60;
        let height = 40;

        ppm.header(width, height).unwrap();
    }
}
