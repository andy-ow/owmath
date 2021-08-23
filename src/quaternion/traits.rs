pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self: f32) -> f32 {
        self.sqrt()
    }
}

impl Sqrt for f64 {
    fn sqrt(self: f64) -> f64 {
        self.sqrt()
    }
}

pub trait TQ = std::ops::Mul<Self, Output = Self>
    //+ Sized + std::ops::Add<Self, Output = Self> + num_traits::Zero
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Neg<Output = Self>
    + std::fmt::Display
    + PartialEq
    + Sqrt
    + Copy;
