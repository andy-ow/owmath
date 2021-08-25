use crate::quaternion::traits::Field;
use crate::Quaternion;
use duplicate::duplicate;

impl<T: Field> std::ops::Neg for Quaternion<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Quaternion::new(-self.r, -self.i, -self.j, -self.k)
    }
}

impl<T: Field> std::ops::Add for Quaternion<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Quaternion::new(
            self.r + rhs.r,
            self.i + rhs.i,
            self.j + rhs.j,
            self.k + rhs.k,
        )
    }
}

impl<T: Field> std::ops::Sub for Quaternion<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Quaternion::new(
            self.r - rhs.r,
            self.i - rhs.i,
            self.j - rhs.j,
            self.k - rhs.k,
        )
    }
}

impl<T: Field> std::ops::Mul<T> for Quaternion<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Quaternion::new(self.r * rhs, self.i * rhs, self.j * rhs, self.k * rhs)
    }
}

#[duplicate(TYPE; [f32]; [f64])]
impl std::ops::Mul<Quaternion<TYPE>> for TYPE {
    type Output = Quaternion<TYPE>;
    fn mul(self, rhs: Quaternion<TYPE>) -> Quaternion<TYPE> {
        rhs * self
    }
}

impl<T: Field> std::ops::Mul<Quaternion<T>> for Quaternion<T> {
    type Output = Quaternion<T>;
    fn mul(self, rhs: Quaternion<T>) -> Quaternion<T> {
        Quaternion::new(
            self.r * rhs.r - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
            self.r * rhs.i + self.i * rhs.r + self.j * rhs.k - self.k * rhs.j,
            self.r * rhs.j - self.i * rhs.k + self.j * rhs.r + self.k * rhs.i,
            self.r * rhs.k + self.i * rhs.j - self.j * rhs.i + self.k * rhs.r,
        )
    }
}

#[duplicate(TESTS TYPE EPSILON;
[tests_f32] [f32] [0.000001];
[tests_f64] [f64] [0.00000000000001];)]
#[cfg(test)]
mod TESTS {
    use crate::Quaternion;

    #[test]
    fn negative_quaternion() {
        let a: Quaternion<TYPE> = Quaternion::new(-1.0, 2.0, 3.0, 4.0);
        assert_eq!(Quaternion::new(0.0, 0.0, 0.0, 0.0) - a, -a);
        let b: Quaternion<TYPE> = -a;
        assert_eq!(a, -b);
        assert_ne!(a, b);
        assert_eq!(-b, Quaternion::new(-1.0, 2.0, 3.0, 4.0));
        assert_eq!(b, Quaternion::new(1.0, -2.0, -3.0, -4.0));
        assert_ne!(-b, Quaternion::new(1.0, -2.0, -3.0, -4.0));
    }

    #[test]
    fn equality_quaternion() {
        let a: Quaternion<TYPE> = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(a, Quaternion::new(1.0, 2.0, 3.0, 4.0));
        assert_ne!(a, Quaternion::new(1.1, 2.0, 3.0, 4.0));
        assert_ne!(a, Quaternion::new(1.0, -2.0, 3.0, 4.0));
        assert_ne!(a, Quaternion::new(1.0, 2.0, 2.0, 4.0));
        assert_ne!(a, Quaternion::new(1.0, 2.0, 3.0, 5.0));
    }

    #[test]
    fn add_quaternion() {
        let a: Quaternion<TYPE> = Quaternion::new(1.0, 2.0, 3.0, -4.0);
        let b: Quaternion<TYPE> = Quaternion::new(1.1, 1.2, -1.9, -1.9);
        assert_eq!(a + b, Quaternion::new(2.1, 3.2, 1.1, -5.9));
    }

    #[test]
    fn sub_quaternion() {
        let a: Quaternion<TYPE> = Quaternion::new(1.0, 2.0, 3.0, -4.0);
        let b: Quaternion<TYPE> = Quaternion::new(1.0, 1.0, -2.0, -2.0);
        assert_eq!(a - b, Quaternion::new(0.0, 1.0, 5.0, -2.0));
    }

    #[test]
    fn mul_scalar_quaternion() {
        let a: Quaternion<TYPE> = Quaternion::new(1.0, 2.0, 3.0, -4.0);
        let b: TYPE = 5.0;
        assert_eq!(a * b, Quaternion::new(5.0, 10.0, 15.0, -20.0));
        assert_eq!(b * a, Quaternion::new(5.0, 10.0, 15.0, -20.0));
    }

    #[test]
    fn mul_hamilton_mul() {
        let a: Quaternion<TYPE> = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let b: Quaternion<TYPE> = Quaternion::new(2.0, 3.0, 4.0, 5.0);
        assert_eq!(a * b, Quaternion::new(-36.0, 6.0, 12.0, 12.0))
    }
}
