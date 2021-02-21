use std::ops::Range;
use std::collections::{ hash_map::Entry, HashMap };

use crate::{ tree, read };

/// Used with `from_tree` to map graph feature ids to tree feature ids.
#[derive(Debug,PartialEq)]
pub struct Trace {
    bond_id: usize,
    map: HashMap<(usize, usize), usize>,
    opens: HashMap<tree::Rnum, Open>,
    source: read::Trace
}

impl Trace {
    pub fn new(source: read::Trace) -> Self {
        Self {
            bond_id: 0,
            map: HashMap::new(),
            opens: HashMap::new(),
            source
        }
    }

    /// Returns the cursor for the atom identifiedy by `id`.
    pub fn atom_cursor(&self, id: usize) -> Option<Range<usize>> {
        self.source.atoms.get(id).map(|cursor| cursor.start..cursor.end)
    }

    /// Returns the cursor for the bond identified by `sid` and `tid`.
    pub fn bond_cursor(&self, sid: usize, tid: usize) -> Option<usize> {
        match self.map.get(&(sid, tid)) {
            Some(id) => self.source.bonds.get(*id).map(|cursor| *cursor),
            None => None
        }
    }
    
    /// Adds the bond identified by atom ids `sid` and `tid`.
    pub fn bond(&mut self, sid: usize, tid: usize) {
        self.map.insert((sid, tid), self.bond_id);
        self.map.insert((tid, sid), self.bond_id);

        self.bond_id += 1
    }

    /// Adds a join to the atom identified by `sid` and the rnum.
    pub fn join(&mut self, sid: usize, rnum: tree::Rnum) {
        match self.opens.entry(rnum) {
            Entry::Occupied(occupied) => {
                let Open {sid: tid, bid } = occupied.remove();

                self.map.insert((tid, sid), bid);
                self.map.insert((sid, tid), self.bond_id);
            },
            Entry::Vacant(vacant) => {
                vacant.insert(Open { sid, bid: self.bond_id });
            }
        }

        self.bond_id += 1;
    }
}

#[derive(Debug,PartialEq)]
struct Open {
    sid: usize,
    bid: usize
}

#[cfg(test)]
mod bond_id {
    use pretty_assertions::assert_eq;
    use crate::read;
    use super::*;

    #[test]
    fn empty() {
        let trace = Trace::new(read::Trace {
            atoms: vec![ ],
            bonds: vec![ ],
            rnums: vec![ ]
        });

        assert_eq!(trace.bond_cursor(0, 1), None)
    }

    #[test]
    fn p2() {
        let mut trace = Trace::new(read::Trace {
            atoms: vec![ 0..1, 1..2 ],
            bonds: vec![ 1 ],
            rnums: vec![ ]
        });

        trace.bond(0, 1);

        assert_eq!(trace.bond_cursor(0, 1), Some(1));
        assert_eq!(trace.bond_cursor(1, 0), Some(1))
    }

    #[test]
    fn p3() {
        let mut trace = Trace::new(read::Trace {
            atoms: vec![ 0..1, 1..2, 2..3 ],
            bonds: vec![ 1, 2 ],
            rnums: vec![ ]
        });

        trace.bond(0, 1);
        trace.bond(1, 2);

        assert_eq!(trace.bond_cursor(0, 1), Some(1));
        assert_eq!(trace.bond_cursor(1, 0), Some(1));
        assert_eq!(trace.bond_cursor(1, 2), Some(2));
        assert_eq!(trace.bond_cursor(2, 1), Some(2))
    }

    #[test]
    fn c3() {
        let mut trace = Trace::new(read::Trace {
            atoms: vec![ 0..1, 2..3, 3..4 ],
            bonds: vec![ 1, 2, 3, 4 ],
            rnums: vec![ 1..2, 4..5 ]
        });

        trace.join(0, tree::Rnum::R1);
        trace.bond(0, 1);
        trace.bond(1, 2);
        trace.join(2, tree::Rnum::R1);

        assert_eq!(trace.bond_cursor(0, 1), Some(2));
        assert_eq!(trace.bond_cursor(1, 0), Some(2));
        assert_eq!(trace.bond_cursor(1, 2), Some(3));
        assert_eq!(trace.bond_cursor(2, 1), Some(3));
        assert_eq!(trace.bond_cursor(2, 0), Some(4));
        assert_eq!(trace.bond_cursor(0, 2), Some(1))
    }
    
    #[test]
    fn diamond() {
        let mut trace = Trace::new(read::Trace {
            atoms: vec![ 0..1, 3..4, 4..5, 6..7 ],
            bonds: vec![ 1, 2, 3, 4, 5, 6, 7 ],
            rnums: vec![ 1..2, 2..3, 5..6, 7..8 ]
        });

        trace.join(0, tree::Rnum::R1);
        trace.join(0, tree::Rnum::R2);
        trace.bond(0, 1);
        trace.bond(1, 2);
        trace.join(2, tree::Rnum::R1);
        trace.bond(2, 3);
        trace.join(3, tree::Rnum::R2);

        assert_eq!(trace.bond_cursor(0, 3), Some(2));
        assert_eq!(trace.bond_cursor(0, 2), Some(1));
        assert_eq!(trace.bond_cursor(2, 0), Some(5));
        assert_eq!(trace.bond_cursor(3, 0), Some(7))
    }
}