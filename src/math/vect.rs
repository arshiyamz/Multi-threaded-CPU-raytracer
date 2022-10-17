use std::ops::*;
use std::convert::Into;

pub trait VectorableType :
    Default +
    Copy +
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    Into<f64> {}

impl VectorableType for u8 {}
impl VectorableType for u16 {}
impl VectorableType for u32 {}

impl VectorableType for i8 {}
impl VectorableType for i16 {}
impl VectorableType for i32 {}

impl VectorableType for f32 {}
impl VectorableType for f64 {}

#[derive(Debug)]
pub struct Vect<const COUNT: usize, T: VectorableType>
{
    data: [T; COUNT],
}

impl<const COUNT: usize, T: VectorableType> Vect<COUNT, T>
{
    pub fn new() -> Self
    {
        Vect::<COUNT, T>
        {
           data: [T::default(); COUNT],
        }
    }
}

// Non-mutable Accessor:

impl<const COUNT: usize, T: VectorableType> Index<usize> for Vect<COUNT, T>
{
    type Output = T;

    fn index(&self, ind: usize) -> &T
    {
        &self.data[ind]
    }
}

// Mutable Accessor:

impl<const COUNT: usize, T: VectorableType> IndexMut<usize> for Vect<COUNT, T>
{
    fn index_mut(&mut self, ind: usize) -> &mut Self::Output
    {
        &mut self.data[ind]
    }
}

// Convenience Operator Overloads:

impl<'a, 'b, const COUNT: usize, T: VectorableType> Add<&'b Vect::<COUNT, T>> for &'a Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn add(self, other: &'b Vect::<COUNT, T>) -> Self::Output
    {
        let mut result = Vect::<COUNT, T>::new();
        for ind in 0..COUNT
        {
            result[ind] = self.data[ind] + other[ind];
        }
        result
    }
}

impl<'a, 'b, const COUNT: usize, T: VectorableType> Sub<&'b Vect::<COUNT, T>> for &'a Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn sub(self, other: &'b Vect::<COUNT, T>) -> Self::Output
    {
        let mut result = Vect::<COUNT, T>::new();
        for ind in 0..COUNT
        {
            result[ind] = self.data[ind] - other[ind];
        }
        result
    }
}

impl<'a, const COUNT: usize, T: VectorableType> AddAssign<&'a Vect::<COUNT, T>> for Vect<COUNT, T>
{
    fn add_assign(&mut self, other: &'a Vect::<COUNT, T>)
    {
        for ind in 0..COUNT
        {
            self.data[ind] = self.data[ind] + other[ind];
        }
    }
}

impl<'a, const COUNT: usize, T: VectorableType> SubAssign<&'a Vect::<COUNT, T>> for Vect<COUNT, T>
{
    fn sub_assign(&mut self, other: &'a Vect::<COUNT, T>)
    {
        for ind in 0..COUNT
        {
            self.data[ind] = self.data[ind] - other[ind];
        }
    }
}

impl<'a, const COUNT: usize, T: VectorableType> Mul<T> for &'a Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn mul(self, other: T) -> Self::Output
    {
        let mut result = Vect::<COUNT, T>::new();
        for ind in 0..COUNT
        {
            result[ind] = self.data[ind] * other;
        }
        result
    }
}

impl<'a, const COUNT: usize, T: VectorableType> MulAssign<T> for Vect<COUNT, T>
{
    fn mul_assign(&mut self, other: T)
    {
        for ind in 0..COUNT
        {
            self.data[ind] = self.data[ind] * other;
        }
    }
}

impl<'a, const COUNT: usize, T: VectorableType> Div<T> for &'a Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn div(self, other: T) -> Self::Output
    {
        let mut result = Vect::<COUNT, T>::new();
        for ind in 0..COUNT
        {
            result[ind] = self.data[ind] / other;
        }
        result
    }
}

impl<'a, const COUNT: usize, T: VectorableType> DivAssign<T> for Vect<COUNT, T>
{
    fn div_assign(&mut self, other: T)
    {
        for ind in 0..COUNT
        {
            self.data[ind] = self.data[ind] / other;
        }
    }
}

// Length Calculations

impl<const COUNT: usize, T: VectorableType> Vect<COUNT, T>
{
    pub fn length_squared (&self) -> f64
    {
        let mut result = 0f64;
        for ind in 0..COUNT
        {
            result += (self.data[ind] * self.data[ind]).into();
        }
        result
    }

    pub fn length (&self) -> f64
    {
        self.length_squared().sqrt()
    }
}