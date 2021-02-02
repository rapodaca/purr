use crate::parts::BondKind;

/// A bond from a graph-like Atom to an Atom ID.
#[derive(Debug,PartialEq)]
pub struct Bond {
    pub kind: BondKind,
    pub tid: usize
}

impl Bond {
    /// Constructs a Bond.
    pub fn new(kind: BondKind, tid: usize) -> Self {
        Self {
            kind,
            tid
        }
    }

    /// Returns the order of this Bond. Elided, Single, Up, Down,
    /// and Aromatic kinds return 1. The rest return the bond multiplicity.
    pub fn order(&self) -> u8 {
        match &self.kind {
            BondKind::Elided |
            BondKind::Single |
            BondKind::Up |
            BondKind::Down |
            BondKind::Aromatic => 1,
            BondKind::Double => 2,
            BondKind::Triple => 3,
            BondKind::Quadruple => 4
        }
    }

    /// Returns true if this bond is encoded as Aromatic.
    pub fn is_aromatic(&self) -> bool {
        self.kind == BondKind::Aromatic
    }
}