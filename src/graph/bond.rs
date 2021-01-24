use crate::parts::BondKind;

#[derive(Debug,PartialEq)]
pub struct Bond {
    pub kind: BondKind,
    pub tid: usize
}

impl Bond {
    pub fn new(kind: BondKind, tid: usize) -> Self {
        Self {
            kind,
            tid
        }
    }

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

    pub fn is_aromatic(&self) -> bool {
        self.kind == BondKind::Aromatic
    }
}