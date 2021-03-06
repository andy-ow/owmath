mod basic_arithmetic;
pub mod traits;

use crate::quaternion::traits::Field;
use duplicate::duplicate;
use std::fmt::Formatter;
use traits::Sqrt;

/// A quaternion number of the form a + b<b>i</b> + c<b>j</b> + d<b>k</b>.
/// T is either f32, f64 or must implement the following traits:
/// Copy
///     + owmath::quaternion::traits::Sqrt
///     + std::ops::Neg<Output = Self>
///     + std::ops::Add<Output = Self>
///     + std::ops::Sub<Output = Self>
///     + std::ops::Mul<Output = Self>
///     + std::ops::Div<Output = Self>
///
/// ## Example
///
/// ```rust
/// use owmath::quaternion::Quaternion;
///
/// fn main() {
///     let a = Quaternion::new(1.0, 2.0, 3.0, 4.0);
///     let b: Quaternion<f64> = (2.0, 3.0, 4.0, 5.0).into();
///     println!("{} + {} = {}", a, b, a+b);
///     println!("{} * {} = {}", a, b, a*b);
///     println!("{} * {} = {}", b, a, b*a);
///     println!("2 * {} = {}", a, 2.0*a);  // works only for Quaternion<f32> and Quaternion<f64>
///     println!("{} * 2 = {}", a, a*2.0);
///     println!("{}⁻¹ = {}", a, a.inverse());
///     println!("{}.norm() = {}", a, a.norm());
///     println!("{}.conjugate() = {}", a, a.conjugate());
/// }
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion<T> {
    pub r: T,
    pub i: T,
    pub j: T,
    pub k: T,
}

impl<T: Field> Quaternion<T> {
    /// Creates a new quaternion.
    pub fn new(r: T, i: T, j: T, k: T) -> Quaternion<T> {
        Quaternion { r, i, j, k }
    }

    /// Norm. (a, b, c, d).norm() := sqrt(a² + b² + c² + d²)
    pub fn norm(self) -> T {
        Sqrt::sqrt(self.r * self.r + self.i * self.i + self.j * self.j + self.k * self.k)
    }
    /// Conjugate. (a, b, c, d).conjugate() := (a, -b, -c, -d)
    pub fn conjugate(self) -> Quaternion<T> {
        Self::new(self.r, -self.i, -self.j, -self.k)
    }

    /// Inverse. (a, b, c, d).inverse() := (a, b, c, d)⁻¹
    pub fn inverse(self) -> Quaternion<T> {
        let quotient = self.r * self.r + self.i * self.i + self.j * self.j + self.k * self.k;
        Self::new(
            self.r / quotient,
            -self.i / quotient,
            -self.j / quotient,
            -self.k / quotient,
        )
    }
}

#[duplicate(TYPE; [f32]; [f64])]
impl std::convert::From<(TYPE, TYPE, TYPE, TYPE)> for Quaternion<TYPE> {
    fn from(tuple: (TYPE, TYPE, TYPE, TYPE)) -> Quaternion<TYPE> {
        Quaternion::new(tuple.0, tuple.1, tuple.2, tuple.3)
    }
}

impl<T> std::fmt::Display for Quaternion<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.i, self.j, self.k)
    }
}

#[duplicate(TESTS TYPE EPSILON;
[tests_f32] [f32] [0.000001];
[tests_f64] [f64] [0.00000000000001];)]
#[cfg(test)]
mod TESTS {
    use crate::quaternion::Quaternion;
    #[test]
    fn new_test() {
        let _a: Quaternion<TYPE> = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    }

    #[test]
    fn norm_test() {
        let a: Quaternion<TYPE> = Quaternion::new(1.0, -1.0, 3.0, 5.0);
        assert!((a.norm() - 6.0).abs() < EPSILON);
        let b: Quaternion<TYPE> = Quaternion::new(1.1, -1.0, 3.0, 5.0);
        assert!((b.norm() - 6.0).abs() > EPSILON);
    }

    #[test]
    fn conjugate() {
        let a: Quaternion<TYPE> = Quaternion::new(TYPE::into(1.0), 2.0, 3.0, 4.0);
        let b: Quaternion<TYPE> = Quaternion::new(1.0, -2.0, -3.0, -4.0);
        assert_eq!(a.conjugate(), b);
        let c: Quaternion<TYPE> = Quaternion::new(1.2, 2.0, 3.0, 4.0);
        assert_eq!(c.conjugate().conjugate(), c);
    }

    #[test]
    fn inverse() {
        let a: Quaternion<TYPE> = Quaternion::new(2.0, 0.0, 0.0, 0.0);
        assert_eq!(a.inverse(), Quaternion::new(0.5, 0.0, 0.0, 0.0));
        let b: Quaternion<TYPE> = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        assert!((b - b.inverse().inverse()).norm() < EPSILON);
        assert!((Quaternion::new(1.0, 0.0, 0.0, 0.0) - b.inverse() * b).norm() < EPSILON);
    }

    #[test]
    fn convert_tuple() {
        let a: Quaternion<TYPE> = Quaternion::from((1.0, 2.0, 3.0, 4.0));
        assert_eq!(a, Quaternion::new(1.0, 2.0, 3.0, 4.0));
        let b: Quaternion<TYPE> = (1.0, 2.0, 3.0, 4.0).into();
        assert_eq!(a, b);
    }

    #[test]
    fn display_print() {
        let a = Quaternion::new(1.2, 2.0, 3.0, 4.0);
        //println!("{} == (1.2, 2, 3, 4)", &a);
        assert_eq!(
            format!("The origin is: {}", &a),
            "The origin is: (1.2, 2, 3, 4)"
        );
    }
}
