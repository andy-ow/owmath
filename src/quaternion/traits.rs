use duplicate::duplicate;

pub trait Field = Copy
    + Sqrt
    + std::ops::Neg<Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>;

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

#[cfg(not(tarpaulin_include))]
#[duplicate(TYPE; [f32]; [f64])]
impl Sqrt for TYPE {
    fn sqrt(self: TYPE) -> TYPE {
        self.sqrt()
    }
}

#[duplicate(TESTS TYPE; [tests_f32] [f32]; [tests_f64] [f64])]
#[cfg(test)]
mod TESTS {
    #[allow(unused_imports)]
    use crate::quaternion::traits::Sqrt;

    #[test]
    fn sqrt() {
        let a: TYPE = 4.0;
        let b: TYPE = 2.0;
        let c = a.sqrt();
        assert_eq!(b, c);
    }
}