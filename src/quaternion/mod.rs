mod basic_arithmetic;
pub mod traits;

use crate::quaternion::traits::Field;
use duplicate::duplicate;
use std::fmt::Formatter;
use traits::Sqrt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion<T> {
    pub r: T,
    pub i: T,
    pub j: T,
    pub k: T,
}

impl<T: Field> Quaternion<T> {
    pub fn new(r: T, i: T, j: T, k: T) -> Quaternion<T> {
        Quaternion { r, i, j, k }
    }

    pub fn norm(self) -> T {
        Sqrt::sqrt(self.r * self.r + self.i * self.i + self.j * self.j + self.k * self.k)
    }

    pub fn conjugate(self) -> Quaternion<T> {
        Self::new(self.r, -self.i, -self.j, -self.k)
    }

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
    use num_traits::abs;
    # [test]
    fn new_test() {
    let _a: Quaternion<TYPE> = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    }

    # [test]
    fn norm_test() {
    let a: Quaternion<TYPE> = Quaternion::new(1.0, - 1.0, 3.0, 5.0);
    assert ! (abs(a.norm() - 6.0) < EPSILON);
    let b: Quaternion<TYPE> = Quaternion::new(1.1, - 1.0, 3.0, 5.0);
    assert ! (abs(b.norm() - 6.0) > EPSILON);
    }

    # [test]
    fn conjugate() {
    let a: Quaternion<TYPE> = Quaternion::new(TYPE::into(1.0), 2.0, 3.0, 4.0);
    let b: Quaternion<TYPE> = Quaternion::new(1.0, - 2.0, - 3.0, - 4.0);
    assert_eq ! (a.conjugate(), b);
    let c: Quaternion<TYPE> = Quaternion::new(1.2, 2.0, 3.0, 4.0);
    assert_eq ! (c.conjugate().conjugate(), c);
    }

    # [test]
    fn inverse() {
    let a: Quaternion<TYPE> = Quaternion::new(2.0, 0.0, 0.0, 0.0);
    assert_eq ! (a.inverse(), Quaternion::new(0.5, 0.0, 0.0, 0.0));
    let b: Quaternion<TYPE> = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    assert! ((b - b.inverse().inverse()).norm() < EPSILON);
    assert ! ((Quaternion::new(1.0, 0.0, 0.0, 0.0) - b.inverse() * b).norm() < EPSILON);
    }

    # [test]
    fn convert_tuple() {
    let a: Quaternion<TYPE> = Quaternion::from((1.0, 2.0, 3.0, 4.0));
    assert_eq ! (a, Quaternion::new(1.0, 2.0, 3.0, 4.0));
    let b: Quaternion <TYPE> = (1.0, 2.0, 3.0, 4.0).into();
    assert_eq ! (a, b);
    }

    # [test]
    fn display_print() {
    let a = Quaternion::new(1.2, 2.0, 3.0, 4.0);
    //println!("{} == (1.2, 2, 3, 4)", &a);
    assert_eq ! (
    format ! ("The origin is: {}", & a),
    "The origin is: (1.2, 2, 3, 4)"
    );
    }
    }
