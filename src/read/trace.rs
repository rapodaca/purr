use std::ops::Range;
use std::collections::{ HashMap, hash_map::Entry };

use crate::feature::Rnum;

/// Maps the features of an ajacency representation to cursors within a
/// string representation.
#[derive(Debug,PartialEq)]
pub struct Trace {
    atoms: Vec<Range<usize>>,
    bonds: HashMap<(usize, usize), usize>,
    stack: Vec<usize>,
    opens: HashMap<Rnum, Open>,
    rnums: Vec<Range<usize>>
}

impl Trace {
    pub fn new() -> Self {
        Self {
            atoms: Vec::new(),
            bonds: HashMap::new(),
            stack: Vec::new(),
            opens: HashMap::new(),
            rnums: Vec::new()
        }
    }

    /// Returns the cursor range associated with atom identifier `id`.
    pub fn atom(&self, id: usize) -> Option<Range<usize>> {
        self.atoms.get(id).map(|cursor| cursor.clone())
    }

    /// Returns the cursor associated with the bond between
    /// `sid` and `tid`. Usually, this cursor will be the same with
    /// either orientation, but in the case of a join (ring closure),
    /// two different cursors will be reported for (`sid`, `tid`)
    /// and (`tid`, `sid`).
    pub fn bond(&self, sid: usize, tid: usize) -> Option<usize> {
        self.bonds.get(&(sid, tid)).map(|cursor| *cursor)
    }

    /// Returns the `Rnum` associated with ring closure digit
    /// identifier `rid`.
    pub fn rnum(&self, rid: usize) -> Option<Range<usize>> {
        self.rnums.get(rid).map(|cursor| cursor.clone())
    }

    /// Adds a root atom.
    pub fn root(&mut self, cursor: Range<usize>) {
        self.stack.push(self.atoms.len());
        self.atoms.push(cursor)
    }

    /// Extends head.
    pub fn extend(&mut self, bond_cursor: usize, atom_cursor: Range<usize>) {
        let sid = *self.stack.last().expect("last on stack");
        let tid = self.atoms.len();

        self.bonds.insert((sid, tid), bond_cursor);
        self.bonds.insert((tid, sid), bond_cursor);
        self.atoms.push(atom_cursor);
        self.stack.push(tid);
    }

    /// Joins a ring closure to head.
    pub fn join(
        &mut self, bond_cursor: usize, rnum_cursor: Range<usize>, rnum: Rnum
    ) {
        let sid = *self.stack.last().expect("last on stack");

        match self.opens.entry(rnum) {
            Entry::Occupied(occupied) => {
                let open = occupied.remove();

                self.bonds.insert((sid, open.sid), bond_cursor);
                self.bonds.insert((open.sid, sid), open.bond_cursor);
            },
            Entry::Vacant(vacant) => {
                vacant.insert(Open {
                    sid,
                    bond_cursor,
                    rnum_cursor: rnum_cursor.clone()
                });
            }
        }

        self.rnums.push(rnum_cursor);
    }

    /// Pops back by `depth`.
    /// 
    /// # Panics
    /// 
    /// Panics if depth is greater than or equal to the current chain
    /// length.
    pub fn pop(&mut self, depth: usize) {
        if depth >= self.stack.len() {
            panic!("overpop")
        }

        for _ in 0..depth {
            self.stack.pop();
        }
    }
}

#[derive(Debug,PartialEq)]
struct Open {
    sid: usize,
    bond_cursor: usize,
    rnum_cursor: Range<usize>
}

#[cfg(test)]
mod pop {
    use super::*;

    #[test]
    #[should_panic(expected = "overpop")]
    fn overpop() {
        let mut trace = Trace::new();

        trace.root(0..1);
        trace.pop(1)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn p3() {
        let mut trace = Trace::new();

        trace.root(0..1);
        trace.extend(1, 1..2);
        trace.extend(2, 2..3);

        assert_eq!(trace.atom(0), Some(0..1));
        assert_eq!(trace.atom(1), Some(1..2));
        assert_eq!(trace.atom(2), Some(2..3));
        assert_eq!(trace.bond(0, 1), Some(1));
        assert_eq!(trace.bond(1, 0), Some(1));
        assert_eq!(trace.bond(1, 2), Some(2))
    }

    #[test]
    fn p3_branch() {
        let mut trace = Trace::new();

        trace.root(0..1);
        trace.extend(1, 1..2);
        trace.pop(1);
        trace.extend(2, 2..3);

        assert_eq!(trace.atom(0), Some(0..1));
        assert_eq!(trace.atom(1), Some(1..2));
        assert_eq!(trace.atom(2), Some(2..3));
        assert_eq!(trace.bond(0, 1), Some(1));
        assert_eq!(trace.bond(1, 0), Some(1));
        assert_eq!(trace.bond(0, 2), Some(2));
        assert_eq!(trace.bond(2, 0), Some(2))
    }

    #[test]
    fn c3() {
        let mut trace = Trace::new();

        // *1**1
        // 01234
        trace.root(0..1);
        trace.join(1, 1..2, Rnum::R1);
        trace.extend(2, 2..3);
        trace.extend(3, 3..4);
        trace.join(4, 4..5, Rnum::R1);

        assert_eq!(trace.atom(0), Some(0..1));
        assert_eq!(trace.atom(1), Some(2..3));
        assert_eq!(trace.atom(2), Some(3..4));
        assert_eq!(trace.bond(0, 1), Some(2));
        assert_eq!(trace.bond(1, 0), Some(2));
        assert_eq!(trace.bond(1, 2), Some(3));
        assert_eq!(trace.bond(2, 1), Some(3));
        assert_eq!(trace.bond(0, 2), Some(1));
        assert_eq!(trace.bond(2, 0), Some(4));
        assert_eq!(trace.rnum(0), Some(1..2))
    }
}