use std::collections::HashMap;

use crate::{ graph, parts };
use super::{ Atom, join_pool::JoinPool, Link, Target };

/// Returns the root atom in a tree corresponding to the input graph.
pub fn from_graph(graph: Vec<graph::Atom>) -> Atom {
    let mut atoms = graph.into_iter().enumerate().collect::<HashMap<_,_>>();
    let mut links = Vec::new();
    let mut hub = HashMap::new();
    let mut pool = JoinPool::new();
    let mut last = None;

    for id in 0..atoms.len() {
        let atom = match atoms.remove(&id) {
            Some(atom) => atom,
            None => continue
        };

        if let Some(last) = last {
            links.push(Unit::split(last, id))
        }
        
        write_root(id, atom, &mut atoms, &mut links, &mut hub, &mut pool);
        last.replace(id);
    }

    while let Some(unit) = links.pop() {
        unit.add_link(&mut hub)
    }

    hub.remove(&0).expect("result")
}

fn write_root(
    pid: usize,
    parent: graph::Atom,
    atoms: &mut HashMap<usize, graph::Atom>,
    links: &mut Vec<Unit>,
    hub: &mut HashMap<usize, Atom>,
    pool: &mut JoinPool
) {
    let mut stack = Vec::new();

    for bond in parent.bonds {
        stack.push((pid, bond))
    }

    hub.insert(pid, Atom {
        kind: parent.kind,
        links: vec![ ]
    });

    while let Some((sid, bond)) = stack.pop() {
        match atoms.remove(&bond.tid) {
            Some(child) => {
                for out in child.bonds {
                    if out.tid == sid {
                        continue
                    }

                    stack.push((bond.tid, out))
                }

                let mut next = Atom {
                    kind: child.kind,
                    links: vec![ ]
                };

                invert(&mut next.kind);
                hub.insert(bond.tid, next);
                links.push(Unit::bond(sid, bond.kind, bond.tid));
            },
            None => {
                let hit = pool.hit(sid, bond.tid);
                let kind = if hit.closes {
                    bond.kind
                } else {
                    parts::BondKind::Elided
                };

                links.push(Unit::join(sid, kind, hit.rnum))
            }
        }
    }
}

fn invert(kind: &mut parts::AtomKind) {
    if let parts::AtomKind::Bracket { hcount, parity, .. } = kind {
        if let Some(p) = parity {
            if hcount.unwrap_or_default() > 0 {
                if p == &parts::Parity::Clockwise {
                    std::mem::swap(p, &mut parts::Parity::Counterclockwise)
                } else if p == &parts::Parity::Counterclockwise {
                    std::mem::swap(p, &mut parts::Parity::Clockwise)
                }
            }
        }
    }
}

#[derive(Debug,PartialEq)]
enum Unit {
    Bond {
        sid: usize,
        tid: usize,
        kind: parts::BondKind
    },
    Join {
        sid: usize,
        rnum: u16,
        kind: parts::BondKind
    },
    Split {
        sid: usize,
        tid: usize
    }
}

impl Unit {
    fn bond(sid: usize, kind: parts::BondKind, tid: usize) -> Self {
        Self::Bond { sid, kind, tid }
    }

    fn join(sid: usize, kind: parts::BondKind, rnum: u16) -> Self {
        Self::Join { sid, kind, rnum }
    }

    fn split(sid: usize, tid: usize) -> Self {
        Self::Split { sid, tid }
    }

    fn add_link(self, hub: &mut HashMap<usize, Atom>) {
        let (sid, link) = match self {
            Unit::Bond { sid, tid, kind } => (sid, Link::Bond {
                kind,
                target: Target::Atom(hub.remove(&tid).expect("target"))
            }),
            Unit::Join { sid, kind, rnum } => (sid, Link::Bond {
                kind,
                target: Target::Join(rnum)
            }),
            Unit::Split { sid, tid } =>
            (sid, Link::Split(hub.remove(&tid).expect("target")))
        };

        hub.get_mut(&sid).expect("source").links.push(link)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::write::write;
    use crate::read::read;
    use crate::graph::from_tree;
    use super::*;

    #[test]
    fn p1() {
        let graph = from_tree(read("*").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "*")
    }

    #[test]
    fn methane() {
        let graph = from_tree(read("C").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "C")
    }

    #[test]
    fn p1_p1() {
        let graph = from_tree(read("*.*").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "*.*")
    }

    #[test]
    fn methane_ammonia_hydrate() {
        let graph = from_tree(read("C.N.O").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "C.N.O")
    }

    #[test]
    fn methanol() {
        let graph = from_tree(read("CO").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "CO")
    }

    #[test]
    fn propanol() {
        let graph = from_tree(read("CCO").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "CCO")
    }

    #[test]
    fn propanal_explicit_bonds() {
        let graph = from_tree(read("C-C=O").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "C-C=O")
    }

    #[test]
    fn propanol_branched() {
        let graph = from_tree(read("C(O)CC").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "C(O)CC")
    }

    #[test]
    fn trihalo_methane_s3() {
        let graph = from_tree(read("C(F)(Cl)Br").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "C(F)(Cl)Br")
    }

    #[test]
    fn fluoroethanol() {
        let graph = from_tree(read("C(CF)O").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "C(CF)O")
    }

    #[test]
    fn oxirane() {
        let graph = from_tree(read("C1CO1").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "C1CO1")
    }

    #[test]
    fn oxirane_left_single() {
        let graph = from_tree(read("C-1CO1").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "C-1CO1")
    }

    #[test]
    fn oxirane_right_single() {
        let graph = from_tree(read("C1CO-1").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "C-1CO1")
    }

    #[test]
    fn oxirane_left_right_single() {
        let graph = from_tree(read("C-1CO-1").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "C-1CO1")
    }

    #[test]
    fn bicyclobutane() {
        let graph = from_tree(read("C12CC1C2").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "C12CC2C1")
    }

    #[test]
    fn tetrahalomethane_stereocentric() {
        let graph = from_tree(read("[C@](F)(Cl)(Br)I").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "[C@](F)(Cl)(Br)I")
    }

    #[test]
    fn tetrahalomethane_non_stereocentric() {
        let graph = from_tree(read("F[C@](Cl)(Br)I").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "F[C@](Cl)(Br)I")
    }

    #[test]
    fn trihalomethane_stereocentric() {
        let graph = from_tree(read("[C@H](F)(Cl)Br").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "[C@H](F)(Cl)Br")
    }

    #[test]
    fn trihalomethane_hydrate_stereocentric() {
        let graph = from_tree(read("O.[C@H](F)(Cl)Br").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "O.[C@H](F)(Cl)Br")
    }

    #[test]
    fn trihalomethane_non_stereocentric() {
        let graph = from_tree(read("F[C@H](Cl)Br").unwrap().root).unwrap();
        let tree = from_graph(graph);

        assert_eq!(write(&tree), "F[C@H](Cl)Br")
    }
}