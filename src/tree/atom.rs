use crate::parts::{ AtomKind, Aliphatic, Aromatic };
use super::{ Link };

/// An atom in the context of a tree SMILES representation. Outbound
/// connections are available through `links`, but no inbound connection
/// is available. Accessing such information requires either a traversal
/// of the tree or conversion to a graph-like (adjacency) representation.
#[derive(Debug,PartialEq)]
pub struct Atom {
    pub kind: AtomKind,
    pub links: Vec<Link>
}

impl Atom {
    /// Returns a star atom with no links.
    pub fn star() -> Self {
        Self {
            kind: AtomKind::Star,
            links: vec![ ]
        }
    }

    /// Returns an aliphatic atom with no links.
    pub fn aliphatic(aliphatic: Aliphatic) -> Self {
        Self {
            kind: AtomKind::Aliphatic(aliphatic),
            links: vec![ ]
        }
    }

    /// Returns true if the atom was defined as aromatic.
    pub fn aromatic(aromatic: Aromatic) -> Self {
        Self {
            kind: AtomKind::Aromatic(aromatic),
            links: vec![ ]
        }
    }
}