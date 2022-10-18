use std::fmt;

#[derive(Debug, Default)]
pub struct Color
{
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl fmt::Display for Color
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{} {} {}\n", self.r, self.g, self.b)
    }
}

impl Color
{
    pub fn new(r: u8, g: u8, b: u8) -> Self
    {
        Color {
            r,
            g,
            b,
        }
    }
}

