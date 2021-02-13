use std::fmt;

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

impl fmt::Display for BondKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Elided => write!(f, ""),
            Self::Single => write!(f, "-"),
            Self::Double => write!(f, "="),
            Self::Triple => write!(f, "#"),
            Self::Quadruple => write!(f, "$"),
            Self::Up => write!(f, "/"),
            Self::Down => write!(f, "\\"),
            Self::Aromatic => write!(f, ":")
        }
    }
}