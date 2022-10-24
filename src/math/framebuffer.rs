use super::super::utils::color::*;
use std::fs::File;
use std::io::Write;
use std::io::Error;

#[derive(Debug)]
pub struct FrameBuffer<const WIDTH: usize = 1280, const HEIGHT: usize = 720>
{
    pub pixels: Vec<FColor>
}

impl<const WIDTH: usize, const HEIGHT: usize> FrameBuffer<WIDTH, HEIGHT>
{
    pub fn new() -> Self
    {
        FrameBuffer
        {
            pixels: vec![FColor::make_new(0.0, 0.0, 0.0); WIDTH * HEIGHT]
        }
    }

    pub fn write_to_file(&self, file: &mut File) -> Result<(), Error>
    {
        for y in (0..HEIGHT).rev()
        {
            for x in 0..WIDTH
            {
                file.write_all(self.pixels[x + WIDTH * y].display_color().as_bytes())?;
            }
        }
        Ok(())
    }

    pub fn set_at(&mut self, x: usize, y: usize, color: FColor)
    {
        self.pixels[x + WIDTH * y] = color;
    }
}