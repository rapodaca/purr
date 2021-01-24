#[derive(Debug,PartialEq)]
pub enum Parity {
    Clockwise,
    Counterclockwise
}

impl Parity {
    pub fn negate(&self) -> Self {
        match self {
            Parity::Clockwise => Parity::Counterclockwise,
            Parity::Counterclockwise => Parity::Clockwise
        }
    }
}