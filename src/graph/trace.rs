use std::collections::{ hash_map::Entry, HashMap };

use crate::tree;

/// Used with `from_tree` to map graph feature ids to tree feature ids.
#[derive(Debug,PartialEq)]
pub struct Trace {
    id: usize,
    map: HashMap<(usize, usize), usize>,
    opens: HashMap<tree::Rnum, Open>
}

impl Trace {
    pub fn new() -> Self {
        Self {
            id: 0,
            map: HashMap::new(),
            opens: HashMap::new()
        }
    }

    pub fn bond_id(&self, sid: usize, tid: usize) -> Option<usize> {
        self.map.get(&(sid, tid)).map(|id| *id)
    }
    
    pub fn bond(&mut self, sid: usize, tid: usize) {
        self.map.insert((sid, tid), self.id);
        self.map.insert((tid, sid), self.id);

        self.id += 1
    }

    pub fn join(&mut self, sid: usize, rnum: tree::Rnum) {
        match self.opens.entry(rnum) {
            Entry::Occupied(occupied) => {
                let Open {sid: tid, bid } = occupied.remove();

                self.map.insert((tid, sid), bid);
                self.map.insert((sid, tid), self.id);

                self.id += 1
            },
            Entry::Vacant(vacant) => {
                vacant.insert(Open { sid, bid: self.id });
            }
        }

        self.id += 1;
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
    use super::*;

    #[test]
    fn empty() {
        let trace = Trace::new();

        assert_eq!(trace.bond_id(0, 1), None)
    }

    #[test]
    fn p2() {
        let mut trace = Trace::new();

        trace.bond(0, 1);

        assert_eq!(trace.bond_id(0, 1), Some(0));
        assert_eq!(trace.bond_id(1, 0), Some(0))
    }

    #[test]
    fn p3() {
        let mut trace = Trace::new();

        trace.bond(0, 1);
        trace.bond(1, 2);

        assert_eq!(trace.bond_id(0, 1), Some(0));
        assert_eq!(trace.bond_id(1, 0), Some(0));
        assert_eq!(trace.bond_id(1, 2), Some(1));
        assert_eq!(trace.bond_id(2, 1), Some(1))
    }

    #[test]
    fn c3() {
        let mut trace = Trace::new();

        trace.join(0, tree::Rnum::R1);
        trace.bond(0, 1);
        trace.bond(1, 2);
        trace.join(2, tree::Rnum::R1);

        assert_eq!(trace.bond_id(0, 1), Some(1));
        assert_eq!(trace.bond_id(1, 0), Some(1));
        assert_eq!(trace.bond_id(1, 2), Some(2));
        assert_eq!(trace.bond_id(2, 1), Some(2));
        assert_eq!(trace.bond_id(2, 0), Some(3));
        assert_eq!(trace.bond_id(0, 2), Some(0))
    }
}