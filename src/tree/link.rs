use crate::parts::{ AtomKind, BondKind };
use super::{ Target, Atom };

/// A half-bond capturing the kind of bond and its target.
#[derive(Debug,PartialEq)]
pub enum Link {
    Bond {
        kind: BondKind,
        target: Target
    },
    Split(Atom)
}

impl Link {
    pub fn elided_join(rnum: u16) -> Self {
        Link::Bond {
            kind: BondKind::Elided,
            target: Target::Join(rnum)
        }
    }

    pub fn single_join(rnum: u16) -> Self {
        Link::Bond {
            kind: BondKind::Single,
            target: Target::Join(rnum)
        }
    }

    pub fn elided_atom(atom_kind: AtomKind) -> Self {
        Link::Bond {
            kind: BondKind::Elided,
            target: Target::Atom(Atom {
                kind: atom_kind,
                links: vec![ ]
            })
        }
    }

    pub fn single_atom(atom_kind: AtomKind) -> Self {
        Link::Bond {
            kind: BondKind::Single,
            target: Target::Atom(Atom {
                kind: atom_kind,
                links: vec![ ]
            })
        }
    }

    pub fn double_atom(atom_kind: AtomKind) -> Self {
        Link::Bond {
            kind: BondKind::Double,
            target: Target::Atom(Atom {
                kind: atom_kind,
                links: vec![ ]
            })
        }
    }
}