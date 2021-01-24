use crate::parts::{ AtomKind, Aliphatic, Aromatic };
use super::{ Link };

#[derive(Debug,PartialEq)]
pub struct Atom {
    pub kind: AtomKind,
    pub links: Vec<Link>
}

impl Atom {
    pub fn star() -> Self {
        Self {
            kind: AtomKind::Star,
            links: vec![ ]
        }
    }

    pub fn aliphatic(aliphatic: Aliphatic) -> Self {
        Self {
            kind: AtomKind::Aliphatic(aliphatic),
            links: vec![ ]
        }
    }

    pub fn aromatic(aromatic: Aromatic) -> Self {
        Self {
            kind: AtomKind::Aromatic(aromatic),
            links: vec![ ]
        }
    }
}