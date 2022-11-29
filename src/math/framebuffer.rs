use super::super::utils::color::*;
use std::fs::File;
use std::io::Write;
use std::io::Error;

#[derive(Debug)]
pub struct FrameBuffer<const WIDTH: usize = 1280, const HEIGHT: usize = 720>
{
    pub pixels: Vec<Vec<FColor>>
}

impl<const WIDTH: usize, const HEIGHT: usize> FrameBuffer<WIDTH, HEIGHT>
{
    pub fn new() -> Self
    {
        let v = vec![FColor::make_new(0.0, 0.0, 0.0); WIDTH];
        FrameBuffer
        {
            pixels: vec![v; HEIGHT]
        }
    }

    pub fn write_to_file(&self, file: &mut File) -> Result<(), Error>
    {
        for x in (0..HEIGHT).rev()
        {
            for y in 0..WIDTH
            {
                file.write_all(self.pixels[x][y].display_color().as_bytes())?;
            }
        }
        Ok(())
    }

    pub fn set_at(&mut self, x: usize, y: usize, color: FColor)
    {
        self.pixels[x][y] = color;
    }
}

pub struct FrameBufferSlice<'a>
{
    y: usize,
    pub pixels: &'a mut [Vec<FColor>],
}

impl<'a, const WIDTH: usize, const HEIGHT: usize> FrameBuffer<WIDTH, HEIGHT>
{
    pub fn get_slice(&'a mut self, y_start: usize, num_rows: usize) -> FrameBufferSlice<'a>
    {
        FrameBufferSlice::<'a>
        {
            y: y_start,
            pixels: self.pixels.split_at_mut(y_start).1.split_at_mut(num_rows).0,
        }
    }
}