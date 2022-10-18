use std::fmt;
use std::ops::*;
use std::convert::Into;

pub trait VectorableType :
    Default +
    Copy +
    fmt::Display +
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

#[derive(Debug, PartialEq)]
pub struct Vect<const COUNT: usize = 3, T: VectorableType = f64>
{
    pub data: [T; COUNT],
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

impl<const COUNT: usize, T: VectorableType> Clone for Vect<COUNT, T>
{
    fn clone(&self) -> Self
    {
        let mut result = Vect::<COUNT, T>::new();
        for ind in 0..COUNT
        {
            result[ind] = self.data[ind];
        }
        result
    }

    fn clone_from(&mut self, source: &Self)
    {
        for ind in 0..COUNT
        {
            self.data[ind] = source[ind];
        }
    }
}

impl<const COUNT: usize, T: VectorableType> fmt::Display for Vect<COUNT, T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        for ind in 0..(COUNT-1)
        {
            write!(f, "{} ", self.data[ind])?;
        }
        write!(f, "{}", self.data[COUNT-1])
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

impl<const COUNT: usize, T: VectorableType + Neg<Output = T>> Neg for &Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn neg(self) -> Self::Output
    {
        let mut result = Vect::<COUNT, T>::new();
        for ind in 0..COUNT
        {
            result[ind] = -self.data[ind];
        }
        result
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

macro_rules! impl_vect_mul
{
    ( $T:ty ) =>
    {
        impl<'a, const COUNT: usize> Mul<&'a Vect<COUNT, $T>> for $T
        {
            type Output = Vect<COUNT, $T>;

            fn mul(self, other: &'a Vect<COUNT, $T>) -> Self::Output
            {
                other * self
            }
        }
    }
}

impl_vect_mul!(u8);
impl_vect_mul!(u16);
impl_vect_mul!(u32);

impl_vect_mul!(i8);
impl_vect_mul!(i16);
impl_vect_mul!(i32);

impl_vect_mul!(f32);
impl_vect_mul!(f64);

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

macro_rules! impl_vect_div
{
    ( $T:ty ) =>
    {
        impl<'a, const COUNT: usize> Div<&'a Vect<COUNT, $T>> for $T
        {
            type Output = Vect<COUNT, $T>;

            fn div(self, other: &'a Vect<COUNT, $T>) -> Self::Output
            {
                let mut result = Vect::<COUNT, $T>::new();
                for ind in 0..COUNT
                {
                    result[ind] = self / other[ind];
                }
                result
            }
        }
    }
}

impl_vect_div!(u8);
impl_vect_div!(u16);
impl_vect_div!(u32);

impl_vect_div!(i8);
impl_vect_div!(i16);
impl_vect_div!(i32);

impl_vect_div!(f32);
impl_vect_div!(f64);

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

    pub fn dot_with (&self, other: &Vect<COUNT, T>) -> T
    {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2]
    }

    pub fn dot (u: &Vect<COUNT, T>, v: &Vect<COUNT, T>) -> T
    {
        u[0] * v[0] +
        u[1] * v[1] +
        u[2] * v[2]
    }
}

impl<const COUNT: usize, T: VectorableType + From<f64>> Vect<COUNT, T>
{
    pub fn normalize(&mut self)
    {
        *self /= T::from(self.length());
    }

    pub fn get_normalized(&self) -> Vect<COUNT, T>
    {
        let mut result = Vect::<COUNT, T>::new();
        for ind in 0..COUNT
        {
            result[ind] = self.data[ind] / T::from(self.length());
        }
        result
    }
}


impl<T: VectorableType> Vect<3, T>
{
    pub fn cross(u: &Vect<3, T>, v: &Vect<3, T>) -> Vect<3, T>
    {
        Vect::<3, T>
        {
            data: [u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0]],
        }
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
        let vec_default = Vect::new();
        assert_eq!(vec_default, Vect::<3, f64>{data: [0f64; 3]});

        let _vec_u8 = Vect::<5, u8>::new();
        let _vec_u16 = Vect::<5, u16>::new();
        let _vec_u32 = Vect::<5, u32>::new();

        let _vec_i8 = Vect::<5, i8>::new();
        let _vec_i16 = Vect::<5, i16>::new();
        let _vec_i32 = Vect::<5, i32>::new();

        let _vec_f32 = Vect::<5, f32>::new();
        let _vec_f64 = Vect::<5, f64>::new();
    }

    #[test]
    fn display_test()
    {
        let vec = Vect::<7, u8>{data:[1, 2, 3, 4, 5, 6, 7]};
        assert_eq!(format!("{}", vec), "1 2 3 4 5 6 7");
    }

    #[test]
    fn operations_test()
    {
        let u = Vect{data:[1f64, 2f64, 3f64]};
        let v = Vect{data:[4f64, 5f64, 6f64]};

        assert_eq!(&u + &v, Vect{data:[5f64, 7f64, 9f64]});
        assert_eq!(&v + &u, Vect{data:[5f64, 7f64, 9f64]});

        assert_eq!(&u - &v, Vect{data:[-3f64, -3f64, -3f64]});
        assert_eq!(&v - &u, Vect{data:[3f64, 3f64, 3f64]});

        assert_eq!(-&u, Vect{data:[-1f64, -2f64, -3f64]});

        assert_eq!(5f64 * &u, Vect{data:[5f64, 10f64, 15f64]});
        assert_eq!(&u * 5f64, Vect{data:[5f64, 10f64, 15f64]});

        let mut m = Vect{data:[1f64, 2f64, 3f64]};
        m *= 5f64;
        assert_eq!(m, Vect{data:[5f64, 10f64, 15f64]});

        let mut m = Vect{data:[2f64, 4f64, 6f64]};
        assert_eq!(0f64 / &m, Vect{data:[0f64, 0f64, 0f64]});
        assert_eq!(&m / 2f64, Vect{data:[1f64, 2f64, 3f64]});
        m /= 2f64;
        assert_eq!(m, Vect{data:[1f64, 2f64, 3f64]});

        assert_eq!(Vect::dot(&u, &v), 32f64);
        assert_eq!(u.dot_with(&v), 32f64);
        assert_eq!(v.dot_with(&u), 32f64);
        assert_eq!(Vect::cross(&u, &v), Vect{data:[-3f64, 6f64, -3f64]});

        let mut m = Vect{data:[1f64, 0f64, 0f64]};
        assert_eq!(m.get_normalized(), Vect{data:[1f64, 0f64, 0f64]});
        m.normalize();
        assert_eq!(m, Vect{data:[1f64, 0f64, 0f64]});

        assert_eq!(u.length_squared(), 14f64);
        let m = Vect{data:[1f64, 0f64, 0f64]};
        assert_eq!(m.length(), 1f64);
    }

    #[test]
    fn accessors_test()
    {
        let u = Vect{data:[1f64, 2f64, 3f64]};
        let mut v = Vect{data:[4f64, 5f64, 6f64]};

        assert_eq!(u[0], 1f64);
        assert_eq!(u[1], 2f64);
        assert_eq!(u[2], 3f64);

        assert_eq!(v[0], 4f64);
        assert_eq!(v[1], 5f64);
        assert_eq!(v[2], 6f64);

        v[0] = 1f64;
        v[1] = 2f64;
        v[2] = 3f64;

        assert_eq!(v[0], 1f64);
        assert_eq!(v[1], 2f64);
        assert_eq!(v[2], 3f64);
    }
}