use crate::parts::{ AtomKind, BondKind };
use crate::tree::{ Rnum };

pub trait Follower {
    fn root(&mut self, root: &AtomKind);

    fn extend(&mut self, bond_kind: &BondKind, atom_kind: &AtomKind);

    fn join(&mut self, bond_kind: &BondKind, rnum: &Rnum);

    fn pop(&mut self, depth: usize);
}