use crate::feature::{ AtomKind, BondKind, Rnum };

/// The actions possible when traversing a SMILES representation
pub trait Follower {
    /// A root atom has been found. This occurs at the first atom of every
    /// connected component. As such, every use of `Follower` must
    /// begin with at least one call to `root`. But `root` can also be called
    /// after the first atom has been found, as in methane hydrate (`C.O`).
    fn root(&mut self, root: AtomKind);

    /// A bond between the current head atom and the next head atom has
    /// been found. Using this method implies the existence of a head atom,
    /// as in methanol (`C-O` or `CO`).
    /// 
    /// # Panics
    /// 
    /// Panics if headless.
    fn extend(&mut self, bond_kind: BondKind, atom_kind: AtomKind);

    /// A bond between the current head atom and a ring closure digit has
    /// been found. Using this method implies the existence of a head atom,
    /// as in cyclopropane (`C1CC1`).
    /// 
    /// # Panics
    /// 
    /// Panics if headless.
    fn join(&mut self, bond_kind: BondKind, rnum: Rnum);

    /// Pop the stack by the indicated depth. As roots and extensions are
    /// encountered, `Follower` builds a working path. Branching removes
    /// one or more atoms from the head of this path, exposing a new head.
    /// The newly-exposed head will have previously been a head.
    /// 
    /// # Panics
    /// 
    /// Panics given depth exceeds the length of the current path.
    fn pop(&mut self, depth: usize);
}