/// A kind of bond. Elided bonds are not present in the corresponding
/// string representation.
#[derive(Debug,PartialEq,Clone)]
pub enum BondKind {
    Elided,
    Single,
    Double,
    Triple,
    Quadruple,
    Aromatic,
    Up,
    Down
}

impl BondKind {
    /// Directional bonds (Up and Down) return the complementary item.
    /// Everything else returns self.
    pub fn reverse(&self) -> Self {
        match self {
            Self::Elided => Self::Elided,
            Self::Single => Self::Single,
            Self::Double => Self::Double,
            Self::Triple => Self::Triple,
            Self::Quadruple => Self::Quadruple,
            Self::Aromatic => Self::Aromatic,
            Self::Up => Self::Down,
            Self::Down => Self::Up
        }
    }
}