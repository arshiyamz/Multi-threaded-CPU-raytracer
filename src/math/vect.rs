use std::ops::*;

pub trait VectorableType<T> :
    Default +
    Copy +
    Add<Output = T> +
    Sub<Output = T> {}

impl VectorableType<u8> for u8 {}
impl VectorableType<u16> for u16 {}
impl VectorableType<u32> for u32 {}
impl VectorableType<u64> for u64 {}
impl VectorableType<usize> for usize {}

impl VectorableType<i8> for i8 {}
impl VectorableType<i16> for i16 {}
impl VectorableType<i32> for i32 {}
impl VectorableType<i64> for i64 {}
impl VectorableType<isize> for isize {}

impl VectorableType<f32> for f32 {}
impl VectorableType<f64> for f64 {}

#[derive(Debug)]
pub struct Vect<const COUNT: usize, T: VectorableType<T>>
{
    data: [T; COUNT],
}

impl<const COUNT: usize, T: VectorableType<T>> Vect<COUNT, T>
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

impl<const COUNT: usize, T: VectorableType<T>> Index<usize> for Vect<COUNT, T>
{
    type Output = T;

    fn index(&self, ind: usize) -> &T
    {
        &self.data[ind]
    }
}

// Mutable Accessor:

impl<const COUNT: usize, T: VectorableType<T>> IndexMut<usize> for Vect<COUNT, T>
{
    fn index_mut(&mut self, ind: usize) -> &mut Self::Output
    {
        &mut self.data[ind]
    }
}

// Convenience Operator Overloads:

impl<'a, 'b, const COUNT: usize, T: VectorableType<T>> Add<&'b Vect::<COUNT, T>> for &'a Vect<COUNT, T>
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

impl<'a, 'b, const COUNT: usize, T: VectorableType<T>> Sub<&'b Vect::<COUNT, T>> for &'a Vect<COUNT, T>
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

