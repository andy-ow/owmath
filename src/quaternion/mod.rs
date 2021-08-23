mod basic_arithmetic;
pub mod traits;
use std::fmt::Formatter;
use traits::Sqrt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion<T> {
    pub r: T,
    pub i: T,
    pub j: T,
    pub k: T,
}

impl<T> Quaternion<T> {
    pub fn new(r: T, i: T, j: T, k: T) -> Quaternion<T> {
        Quaternion { r, i, j, k }
    }
}
impl<T: Copy + Sqrt + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + std::ops::Sub<Output = T>> Quaternion<T> {
    pub fn norm(self) -> T {
        Sqrt::sqrt(self.r * self.r + self.i * self.i + self.j * self.j + self.k * self.k)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Quaternion<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.i, self.j, self.k)
    }
}

#[cfg(test)]
mod tests {
    use num_traits::abs;

    use crate::quaternion::Quaternion;

    #[test]
    fn new_test() {
        let _a = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    }

    #[test]
    fn norm_test() {
        let a = Quaternion::new(1.0, -1.0, 3.0, 5.0);
        assert!(abs(a.norm() - 6.0) < 0.0000001);
        let b = Quaternion::new(1.1, -1.0, 3.0, 5.0);
        assert!(abs(b.norm() - 6.0) > 0.0000001);
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
