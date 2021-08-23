pub trait Sqrt<T> {
    fn sqrt(self) -> T;
}

impl Sqrt<f32> for f32 {
    fn sqrt(self: f32) -> f32 {
        self.sqrt()
    }
}

impl Sqrt<f64> for f64 {
    fn sqrt(self: f64) -> f64 {
        self.sqrt()
    }
}

pub trait TQ<T> =
std::ops::Mul<T, Output=T>
//+ Sized + std::ops::Add<Self, Output = Self> + num_traits::Zero
+ std::ops::Add<Output=T>
+ std::ops::Sub<Output=T>
+ std::ops::Neg<Output=T>
+ PartialEq
+ Sqrt<T>
+ Copy;
