use super::super::utils::color::*;
use std::fs::File;
use std::io::Write;
use std::io::Error;

#[derive(Debug)]
pub struct FrameBuffer<const WIDTH: usize = 1280, const HEIGHT: usize = 720>
{
    pixels: Vec<Vec<FColor>>
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

impl<'a, const WIDTH: usize, const HEIGHT: usize> FrameBuffer<WIDTH, HEIGHT>
{
    pub fn get_slice(&'a mut self, x_start: usize, x_end: usize, y_start: usize, y_end: usize) -> Vec<&'a mut [FColor]>
    {
        let mut v = Vec::<&'a mut [FColor]>::with_capacity(y_end - y_start);
        for i in y_start..y_end
        {
            unsafe
            {
                let m: &mut Vec<FColor> = &mut *(self.pixels.get_unchecked_mut(i) as *mut Vec<FColor>);
                v.push(&mut m[x_start..x_end]);
            }
        }
        v
    }
}