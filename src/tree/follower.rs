use crate::parts::{ AtomKind, BondKind };

pub trait Follower {
    fn extend(&mut self, bond_kind: &BondKind, atom_kind: &AtomKind);

    fn join(&mut self, bond_kind: &BondKind, rnum: u16);

    fn split(&mut self, atom_kind: &AtomKind);

    fn back(&mut self);
}