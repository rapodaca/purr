use std::collections::{ hash_map::Entry, HashMap };

use crate::{ tree, parts };
use super::Error;

/// Returns a map enabling graph bonds to be related to tree bonds. Graph bonds
/// are identified by a source id, target id tuple. This implementation
/// accounts for ring closures. If uf ring ring closures are unbalanced,
/// returns the Rnum index to the first one.
pub fn map_bonds(tree: &tree::Atom) -> Result<HashMap<(usize, usize), usize>, Error> {
    let mut mapper = Mapper::new();

    tree::walk(tree, &mut mapper);

    mapper.build()
}

struct Join {
    sid: usize,
    bid: usize,
    rid: usize
}

struct Mapper {
    atom_id: usize,
    bond_id: usize,
    rnum_id: usize,
    path: Vec<usize>,
    joins: HashMap<tree::Rnum, Join>,
    map: HashMap<(usize, usize), usize>
}

impl Mapper {
    fn new() -> Self {
        Self {
            atom_id: 0,
            bond_id: 0,
            rnum_id: 0,
            path: Vec::new(),
            joins: HashMap::new(),
            map: HashMap::new()
        }
    }

    fn build(self) -> Result<HashMap<(usize, usize), usize>, Error> {
        let mut unmatched = self.joins.values().map(|join| join.rid)
            .collect::<Vec<_>>();
        
        unmatched.sort();

        if let Some(rid) = unmatched.get(0) {
            return Err(Error::UnbalancedRnum(*rid))
        }

        Ok(self.map)
    }
}

impl tree::Follower for Mapper {
    fn root(&mut self, _: &parts::AtomKind) {
        self.path.push(self.atom_id);

        self.atom_id += 1
    }

    fn extend(&mut self, _: &parts::BondKind, _: &parts::AtomKind) {
        let sid = *self.path.last().expect("last in path");
        let tid = self.atom_id;

        self.map.insert((sid, tid), self.bond_id);
        self.map.insert((tid, sid), self.bond_id);
        self.path.push(tid);

        self.atom_id += 1;
        self.bond_id += 1;
    }

    fn join(&mut self, _: &parts::BondKind, rnum: &tree::Rnum) {
        let sid = *self.path.last().expect("last in path");

        match self.joins.entry(rnum.clone()) {
            Entry::Occupied(occupied) => {
                let Join { sid: tid, bid, rid } = occupied.remove();

                self.map.insert((sid, tid), bid);
                self.map.insert((tid, sid), bid);

                self.rnum_id += 1
            },
            Entry::Vacant(vacant) => {
                vacant.insert( Join {
                    sid,
                    bid: self.bond_id,
                    rid: self.rnum_id
                });

                self.bond_id += 1;
                self.rnum_id += 1
            }
        }
    }

    fn pop(&mut self, depth: usize) {
        for _ in 0..depth {
            self.path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::read::read;
    use super::*;

    #[test]
    fn unbalanced_rnum_single() {
        let root = read("*1*", None).unwrap();

        assert_eq!(map_bonds(&root), Err(Error::UnbalancedRnum(0)))
    }

    #[test]
    fn unbalanced_rnum_first() {
        let root = read("*1**1*2**3**", None).unwrap();

        assert_eq!(map_bonds(&root), Err(Error::UnbalancedRnum(2)))
    }

    #[test]
    fn unbalanced_rnum_last() {
        let root = read("**1**1*2**", None).unwrap();

        assert_eq!(map_bonds(&root), Err(Error::UnbalancedRnum(2)))
    }

    #[test]
    fn p1() {
        let root = read("*", None).unwrap();

        assert_eq!(map_bonds(&root).unwrap().is_empty(), true)
    }

    #[test]
    fn p2() {
        let root = read("**", None).unwrap();

        assert_eq!(map_bonds(&root), Ok(vec![
            ((0, 1), 0),
            ((1, 0), 0)
        ].into_iter().collect::<HashMap<_,_>>()))
    }

    #[test]
    fn p3() {
        let root = read("***", None).unwrap();

        assert_eq!(map_bonds(&root), Ok(vec![
            ((0, 1), 0),
            ((1, 0), 0),
            ((1, 2), 1),
            ((2, 1), 1)
        ].into_iter().collect::<HashMap<_,_>>()))
    }

    #[test]
    fn p3_branched() {
        let root = read("*(*)*", None).unwrap();

        assert_eq!(map_bonds(&root), Ok(vec![
            ((0, 1), 0),
            ((1, 0), 0),
            ((0, 2), 1),
            ((2, 0), 1)
        ].into_iter().collect::<HashMap<_,_>>()))
    }

    #[test]
    fn c3() {
        //               0 12
        let root = read("*1**1", None).unwrap();

        assert_eq!(map_bonds(&root), Ok(vec![
            ((0, 2), 0),
            ((2, 0), 0),
            ((0, 1), 1),
            ((1, 0), 1),
            ((1, 2), 2),
            ((2, 1), 2),
        ].into_iter().collect::<HashMap<_,_>>()))
    }

    #[test]
    fn diamond() {
        //               0  12 3
        let root = read("*12**1*2", None).unwrap();

        assert_eq!(map_bonds(&root), Ok(vec![
            ((0, 2), 0),
            ((2, 0), 0),
            ((0, 3), 1),
            ((3, 0), 1),
            ((0, 1), 2),
            ((1, 0), 2),
            ((1, 2), 3),
            ((2, 1), 3),
            ((2, 3), 4),
            ((3, 2), 4)
        ].into_iter().collect::<HashMap<_,_>>()))
    }
}