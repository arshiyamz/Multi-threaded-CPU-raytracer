use std::fmt;

#[derive(Debug, Default, PartialEq)]
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

    pub fn r(&self) -> u8
    {
        self.r
    }

    pub fn g(&self) -> u8
    {
        self.g
    }

    pub fn b(&self) -> u8
    {
        self.b
    }
}

//============================================
//============================================
//===============Unit Tests===================
//============================================
//============================================

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn creation_test()
    {
        let c = Color::new(10, 20, 30);
        assert_eq!(c, Color{r:10, g:20, b:30});
    }

    #[test]
    fn mutate_test()
    {
        let mut c = Color::new(10, 20, 30);
        c.r = 20;
        c.g = 30;
        c.b = 40;
        assert_eq!(c, Color{r:20, g:30, b:40});
    }
}

