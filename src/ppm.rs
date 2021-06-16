use std::io::prelude::*;

#[derive(Debug)]
pub struct PPM<W: Write> {
    writer: W,
}

impl<W: Write> PPM<W> {
    pub fn new(writer: W) -> Self {
        PPM { writer }
    }

    pub fn header(&mut self, width: u32, height: u32) -> std::io::Result<()> {
        write!(self.writer, "P3\n{} {}\n255\n", width, height)
    }

    pub fn append(&mut self, point_str: &str) -> std::io::Result<()> {
        writeln!(self.writer, "{}", point_str)
    }
}
