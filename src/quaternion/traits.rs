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

#[duplicate(TYPE; [f32]; [f64])]
impl Sqrt for TYPE {
    fn sqrt(self: TYPE) -> TYPE {
        self.sqrt()
    }
}
