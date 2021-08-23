pub mod traits;
mod basic_arithmetic;
use traits::TQ;
use traits::Sqrt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion<T: TQ<T>> {
    pub r: T,
    pub i: T,
    pub j: T,
    pub k: T,
}

impl<T: TQ<T>> Quaternion<T>
{
    pub fn new(r: T, i: T, j: T, k: T) -> Quaternion<T> {
        Quaternion {
            r,
            i,
            j,
            k,
        }
    }

    pub fn norm(self) -> T {
        Sqrt::sqrt(self.r*self.r + self.i*self.i + self.j*self.j + self.k*self.k)
    }

    /*pub fn zero() -> Quaternion<T> {
        Quaternion::new(T::zero(), T::zero(), T::zero(), T::zero())
    }*/
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
}