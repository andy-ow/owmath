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
