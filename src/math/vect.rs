use std::fmt;
use std::ops::*;
use std::convert::Into;
use std::cmp::min_by;

use super::random::*;
use super::core::FLOAT_MARGIN_OF_ERROR;

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

#[derive(Debug, PartialEq, Copy)]
pub struct Vect<const COUNT: usize = 3, T: VectorableType = f64>
{
    pub data: [T; COUNT],
}

impl<T: VectorableType> Vect<3, T>
{
    pub fn make_new(x: T, y: T, z: T) -> Self
    {
        Vect
        {
            data: [x, y, z]
        }
    }
    pub fn x(&self) -> T
    {
        self.data[0]
    }

    pub fn y(&self) -> T
    {
        self.data[1]
    }

    pub fn z(&self) -> T
    {
        self.data[2]
    }

    pub fn get_x(&mut self) -> &mut T
    {
        &mut self.data[0]
    }

    pub fn get_y(&mut self) -> &mut T
    {
        &mut self.data[1]
    }

    pub fn get_z(&mut self) -> &mut T
    {
        &mut self.data[2]
    }

    pub fn r(&self) -> T
    {
        self.data[0]
    }

    pub fn g(&self) -> T
    {
        self.data[1]
    }

    pub fn b(&self) -> T
    {
        self.data[2]
    }

    pub fn get_r(&mut self) -> &mut T
    {
        &mut self.data[0]
    }

    pub fn get_g(&mut self) -> &mut T
    {
        &mut self.data[1]
    }

    pub fn get_b(&mut self) -> &mut T
    {
        &mut self.data[2]
    }
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

    pub fn reset(&mut self)
    {
        for ind in 0..COUNT
        {
            self.data[ind] = T::default();
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
            result[ind] = self.data[ind] + other.data[ind];
        }
        result
    }
}

impl<'a, const COUNT: usize, T: VectorableType> Add<Vect::<COUNT, T>> for &'a Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn add(self, mut other: Vect::<COUNT, T>) -> Self::Output
    {
        for ind in 0..COUNT
        {
            other[ind] = self.data[ind] + other.data[ind];
        }
        other
    }
}

impl<'a, const COUNT: usize, T: VectorableType> Add<&'a Vect::<COUNT, T>> for Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn add(mut self, other: &'a Vect::<COUNT, T>) -> Self::Output
    {
        for ind in 0..COUNT
        {
            self.data[ind] = self.data[ind] + other.data[ind];
        }
        self
    }
}

impl<const COUNT: usize, T: VectorableType> Add<Vect::<COUNT, T>> for Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn add(mut self, other: Vect::<COUNT, T>) -> Self::Output
    {
        for ind in 0..COUNT
        {
            self.data[ind] = self.data[ind] + other.data[ind];
        }
        self
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
            result[ind] = self.data[ind] - other.data[ind];
        }
        result
    }
}

impl<'a, const COUNT: usize, T: VectorableType> Sub<Vect::<COUNT, T>> for &'a Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn sub(self, mut other: Vect::<COUNT, T>) -> Self::Output
    {
        for ind in 0..COUNT
        {
            other.data[ind] = self.data[ind] - other.data[ind];
        }
        other
    }
}

impl<'a, const COUNT: usize, T: VectorableType> Sub<&'a Vect::<COUNT, T>> for Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn sub(mut self, other: &'a Vect::<COUNT, T>) -> Self::Output
    {
        for ind in 0..COUNT
        {
            self.data[ind] = self.data[ind] - other.data[ind];
        }
        self
    }
}

impl<const COUNT: usize, T: VectorableType> Sub<Vect::<COUNT, T>> for Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn sub(mut self, other: Vect::<COUNT, T>) -> Self::Output
    {
        for ind in 0..COUNT
        {
            self.data[ind] = self.data[ind] - other.data[ind];
        }
        self
    }
}

impl<'a, 'b, const COUNT: usize, T: VectorableType> Mul<&'b Vect<COUNT, T>> for &'a Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn mul(self, other: &'b Vect<COUNT, T>) -> Self::Output
    {
        let mut result = Vect::<COUNT, T>::new();
        for ind in 0..COUNT
        {
            result[ind] = self.data[ind] * other[ind];
        }
        result
    }
}

impl<'a, const COUNT: usize, T: VectorableType> Mul<Vect<COUNT, T>> for &'a Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn mul(self, mut other: Vect<COUNT, T>) -> Self::Output
    {
        for ind in 0..COUNT
        {
            other.data[ind] = self.data[ind] * other.data[ind];
        }
        other
    }
}

impl<'a, const COUNT: usize, T: VectorableType> Mul<&'a Vect<COUNT, T>> for Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn mul(mut self, other: &'a Vect<COUNT, T>) -> Self::Output
    {
        for ind in 0..COUNT
        {
            self.data[ind] = self.data[ind] * other.data[ind];
        }
        self
    }
}

impl<const COUNT: usize, T: VectorableType> Mul<Vect<COUNT, T>> for Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn mul(mut self, other: Vect<COUNT, T>) -> Self::Output
    {
        for ind in 0..COUNT
        {
            self.data[ind] = self.data[ind] * other.data[ind];
        }
        self
    }
}

impl<'a, 'b, const COUNT: usize, T: VectorableType> Div<&'b Vect<COUNT, T>> for &'a Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn div(self, other: &'b Vect<COUNT, T>) -> Self::Output
    {
        let mut result = Vect::<COUNT, T>::new();
        for ind in 0..COUNT
        {
            result.data[ind] = self.data[ind] / other.data[ind];
        }
        result
    }
}

impl<'a, const COUNT: usize, T: VectorableType> Div<Vect<COUNT, T>> for &'a Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn div(self, mut other: Vect<COUNT, T>) -> Self::Output
    {
        for ind in 0..COUNT
        {
            other.data[ind] = self.data[ind] / other.data[ind];
        }
        other
    }
}

impl<'a, const COUNT: usize, T: VectorableType> Div<&'a Vect<COUNT, T>> for Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn div(mut self, other: &'a Vect<COUNT, T>) -> Self::Output
    {
        for ind in 0..COUNT
        {
            self.data[ind] = self.data[ind] / other[ind];
        }
        self
    }
}

impl<const COUNT: usize, T: VectorableType> Div<Vect<COUNT, T>> for Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn div(mut self, other: Vect<COUNT, T>) -> Self::Output
    {
        for ind in 0..COUNT
        {
            self.data[ind] = self.data[ind] / other.data[ind];
        }
        self
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

impl<const COUNT: usize, T: VectorableType> AddAssign<Vect::<COUNT, T>> for Vect<COUNT, T>
{
    fn add_assign(&mut self, other: Vect::<COUNT, T>)
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

impl<const COUNT: usize, T: VectorableType> SubAssign<Vect::<COUNT, T>> for Vect<COUNT, T>
{
    fn sub_assign(&mut self, other: Vect::<COUNT, T>)
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

impl<const COUNT: usize, T: VectorableType + Neg<Output = T>> Neg for Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn neg(mut self) -> Self::Output
    {
        for ind in 0..COUNT
        {
            self.data[ind] = -self.data[ind];
        }
        self
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

impl<const COUNT: usize, T: VectorableType> Mul<T> for Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn mul(mut self, other: T) -> Self::Output
    {
        for ind in 0..COUNT
        {
            self.data[ind] = self.data[ind] * other;
        }
        self
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

        impl<const COUNT: usize> Mul<Vect<COUNT, $T>> for $T
        {
            type Output = Vect<COUNT, $T>;

            fn mul(self, other: Vect<COUNT, $T>) -> Self::Output
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

impl<const COUNT: usize, T: VectorableType> MulAssign<T> for Vect<COUNT, T>
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
            result.data[ind] = self.data[ind] / other;
        }
        result
    }
}

impl<const COUNT: usize, T: VectorableType> Div<T> for Vect<COUNT, T>
{
    type Output = Vect<COUNT, T>;

    fn div(mut self, other: T) -> Self::Output
    {
        for ind in 0..COUNT
        {
            self.data[ind] = self.data[ind] / other;
        }
        self
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
                    result.data[ind] = self / other[ind];
                }
                result
            }
        }

        impl<const COUNT: usize> Div<Vect<COUNT, $T>> for $T
        {
            type Output = Vect<COUNT, $T>;

            fn div(self, mut other: Vect<COUNT, $T>) -> Self::Output
            {
                for ind in 0..COUNT
                {
                    other.data[ind] = self / other[ind];
                }
                other
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

impl<const COUNT: usize, T: VectorableType> DivAssign<T> for Vect<COUNT, T>
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

impl Vect<3, f64>
{
    pub fn rand() -> Self
    {
        let mut rand_gen = SimpleDeterministicRandomGenerator::new();
        Vect
        {
            data: [rand_gen.rand(), rand_gen.rand(), rand_gen.rand()]
        }
    }

    pub fn rand_between(min: f64, max: f64) -> Self
    {
        let mut rand_gen = SimpleDeterministicRandomGenerator::new();
        Vect
        {
            data: [rand_gen.rand_between(min, max), rand_gen.rand_between(min, max), rand_gen.rand_between(min, max)]
        }
    }

    pub fn rand_in_sphere() -> Self
    {
        let mut p = Vect::rand_between(-1.0, 1.0);
        loop
        {
            if p.length_squared() < 1.0
            {
                break;
            }
            p = Vect::rand_between(-1.0, 1.0);
        }
        p
    }

    pub fn sqrt(&self) -> Self
    {
        Vect
        {
            data: [self.data[0].sqrt(),
                   self.data[1].sqrt(),
                   self.data[2].sqrt(),]
        }
    }

    pub fn random_unit_vect() -> Self
    {
        Self::rand_in_sphere().get_normalized()
    }

    pub fn random_in_hemisphere(normal: &Vect) -> Self
    {
        let mut result = Self::rand_in_sphere();
        if Self::dot(&result, &normal) < 0.0
        {
            result = - &result;
        }
        result
    }

    pub fn random_in_disk(radius: f64) -> Self
    {
        let mut rand = SimpleDeterministicRandomGenerator::new();
        let mut v = Self::rand_between(-1.0, 1.0);
        *v.get_z() = 0.0;
        v.normalize();
        radius * v * rand.rand()
    }

    pub fn is_zero(&self) -> bool
    {
        self.length() < FLOAT_MARGIN_OF_ERROR
    }

    pub fn reflect(v: &Vect, u: &Vect) -> Vect
    {
        v - (2.0 * Vect::dot(v, u) * u)
    }

    pub fn refract(v: &Vect, normal: &Vect, etai_over_etat: f64) -> Vect
    {
        let normal = normal.get_normalized();
        let v = v.get_normalized();
        let cos_theta = min_by(Vect::dot(&-v, &normal), 1.0, |a, b| a.partial_cmp(b).unwrap());
        let perpendicular = etai_over_etat * (v + cos_theta * normal);
        let parallel = -(((1.0 - perpendicular.length_squared()).abs()).sqrt()) * normal;
        perpendicular + parallel
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
    fn refract_test()
    {
        let u = Vect{data:[1f64, 0f64, 0f64]};
        let v = Vect{data:[1f64, 0f64, 0f64]};

        let res = Vect{data:[-1f64, 0f64, 0f64]};
        assert_eq!(Vect::refract(&u, &v, 1.0), res);

        let u = Vect{data:[1f64, 2f64, 3f64]};
        let v = Vect{data:[4f64, 7f64, 11f64]};

        let res = Vect{data:[-0.267261f64, 0.534522f64, 0.801784f64]};
        assert_eq!(Vect::refract(&u, &v, 8.0), res);
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

        let mut u1 = Vect{data:[1f64, 2f64, 3f64]};
        let v1 = Vect{data:[4f64, 5f64, 6f64]};

        u1 += &v1;

        assert_eq!(u1, Vect{data:[5f64, 7f64, 9f64]});

        u1 -= &v1;

        assert_eq!(u1, Vect{data:[1f64, 2f64, 3f64]});

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