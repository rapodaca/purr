#[derive(Debug,PartialEq)]
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

    pub fn is_directional(&self) -> bool {
        match self {
            Self::Up |
            Self::Down => true,
            _ => false
        }
    }
}