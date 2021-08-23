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
