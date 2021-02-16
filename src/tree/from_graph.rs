use std::collections::HashMap;

use crate::{ graph, parts };
use super::{ Atom, join_pool::JoinPool, Link, Target, Error };

/// Returns the root atom in a tree corresponding to the input graph.
pub fn from_graph(graph: Vec<graph::Atom>) -> Result<Atom, Error> {
    let order = graph.len();
    let mut atoms = graph.into_iter().enumerate().collect::<HashMap<_,_>>();
    let mut units = Vec::new();
    let mut hub = HashMap::new();
    let mut last = None;

    for id in 0..atoms.len() {
        let atom = match atoms.remove(&id) {
            Some(atom) => atom,
            None => continue
        };

        if let Some(last) = last {
            units.push(Unit::split(last, id))
        }
        
        write_root(id, atom, &mut atoms, &mut units, &mut hub, order)?;
        last.replace(id);
    }

    build_tree(units, hub)
}

fn build_tree(
    mut links: Vec<Unit>, mut hub: HashMap<usize, Atom>
) -> Result<Atom, Error> {
    let mut pool = JoinPool::new();

    while let Some(unit) = links.pop() {
        unit.add_link(&mut hub, &mut pool)
    }

    Ok(hub.remove(&0).expect("result"))
}

fn write_root(
    pid: usize,
    parent: graph::Atom,
    atoms: &mut HashMap<usize, graph::Atom>,
    units: &mut Vec<Unit>,
    hub: &mut HashMap<usize, Atom>,
    order: usize
) -> Result<(), Error> {
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
                        if bond_kinds_match(&out.kind, &bond.kind) {
                            continue
                        } else {
                            return Err(Error::TargetMismatch(bond.tid, pid))
                        }
                    }

                    stack.push((bond.tid, out))
                }

                let mut next = Atom {
                    kind: child.kind,
                    links: vec![ ]
                };

                invert(&mut next.kind);
                hub.insert(bond.tid, next);
                units.push(Unit::bond(sid, bond.kind, bond.tid));
            },
            None => {
                if bond.tid >= order {
                    return Err(Error::TargetMismatch(sid, bond.tid))
                }

                units.push(Unit::join(sid, bond.kind, bond.tid))
            }
        }
    }

    Ok(())
}

fn invert(kind: &mut parts::AtomKind) {
    if let parts::AtomKind::Bracket { hcount, configuration, .. } = kind {
        if let Some(p) = configuration {
            match hcount {
                Some(hcount) => if !hcount.is_zero() {
                    if p == &parts::Configuration::TH2 {
                        std::mem::swap(p, &mut parts::Configuration::TH1)
                    } else if p == &parts::Configuration::TH1 {
                        std::mem::swap(p, &mut parts::Configuration::TH2)
                    }
                },
                None => ()
            }
        }
    }
}

fn bond_kinds_match(left: &parts::BondKind, right: &parts::BondKind) -> bool {
    if left == right {
        true
    } else if left == &parts::BondKind::Up && right == &parts::BondKind::Down {
        true
    } else if left == &parts::BondKind::Down && right == &parts::BondKind::Up {
        true
    } else {
        false
    }
}

#[derive(Debug,PartialEq)]
enum Unit {
    Bond {
        sid: usize,
        kind: parts::BondKind,
        tid: usize
    },
    Join {
        sid: usize,
        kind: parts::BondKind,
        tid: usize
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

    fn join(sid: usize, kind: parts::BondKind, tid: usize) -> Self {
        Self::Join { sid, kind, tid }
    }

    fn split(sid: usize, tid: usize) -> Self {
        Self::Split { sid, tid }
    }

    fn add_link(self, hub: &mut HashMap<usize, Atom>, pool: &mut JoinPool) {
        let (sid, link) = match self {
            Unit::Bond { sid, tid, kind } => (sid, Link::Bond {
                kind,
                target: Target::Atom(hub.remove(&tid).expect("target"))
            }),
            Unit::Join { sid, kind, tid } => (sid, Link::Bond {
                kind,
                target: Target::Join(pool.hit(sid, tid).rnum)
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
    use crate::tree::Writer;
    use crate::read::read;
    use crate::graph::from_tree;
    use super::*;

    #[test]
    fn bond_tid_mismatch() {
        let graph = vec![
            graph::Atom {
                kind: parts::AtomKind::Star,
                bonds: vec![
                    graph::Bond::new(parts::BondKind::Elided, 1)
                ]
            },
            graph::Atom {
                kind: parts::AtomKind::Star,
                bonds: vec![
                    graph::Bond::new(parts::BondKind::Elided, 2)
                ]
            }
        ];

        assert_eq!(from_graph(graph), Err(Error::TargetMismatch(1, 2)))
    }

    #[test]
    fn bond_kind_mismatch() {
        let graph = vec![
            graph::Atom {
                kind: parts::AtomKind::Star,
                bonds: vec![
                    graph::Bond::new(parts::BondKind::Elided, 1)
                ]
            },
            graph::Atom {
                kind: parts::AtomKind::Star,
                bonds: vec![
                    graph::Bond::new(parts::BondKind::Double, 0)
                ]
            }
        ];

        assert_eq!(from_graph(graph), Err(Error::TargetMismatch(1, 0)))
    }

    #[test]
    fn p1() {
        let graph = from_tree(read("*").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "*")
    }

    #[test]
    fn methane() {
        let graph = from_tree(read("C").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "C")
    }

    #[test]
    fn p1_p1() {
        let graph = from_tree(read("*.*").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "*.*")
    }

    #[test]
    fn methane_ammonia_hydrate() {
        let graph = from_tree(read("C.N.O").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "C.N.O")
    }

    #[test]
    fn methanol() {
        let graph = from_tree(read("CO").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "CO")
    }

    #[test]
    fn propanol() {
        let graph = from_tree(read("CCO").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "CCO")
    }

    #[test]
    fn propanal_explicit_bonds() {
        let graph = from_tree(read("C-C=O").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "C-C=O")
    }

    #[test]
    fn propanol_branched() {
        let graph = from_tree(read("C(O)CC").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "C(O)CC")
    }

    #[test]
    fn trihalo_methane_s3() {
        let graph = from_tree(read("C(F)(Cl)Br").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "C(F)(Cl)Br")
    }

    #[test]
    fn fluoroethanol() {
        let graph = from_tree(read("C(CF)O").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "C(CF)O")
    }

    #[test]
    fn oxirane() {
        let graph = from_tree(read("C1CO1").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "C1CO1")
    }

    #[test]
    fn oxirane_left_single() {
        let graph = from_tree(read("C-1CO1").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "C-1CO1")
    }

    #[test]
    fn oxirane_right_single() {
        let graph = from_tree(read("C1CO-1").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "C1CO-1")
    }

    #[test]
    fn oxirane_left_right_single() {
        let graph = from_tree(read("C-1CO-1").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "C-1CO-1")
    }

    #[test]
    fn bicyclobutane() {
        let graph = from_tree(read("C12CC1C2").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "C12CC1C2")
    }

    #[test]
    fn tetrahalomethane_stereocentric() {
        let graph = from_tree(read("[C@](F)(Cl)(Br)I").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "[C@](F)(Cl)(Br)I")
    }

    #[test]
    fn tetrahalomethane_non_stereocentric() {
        let graph = from_tree(read("F[C@](Cl)(Br)I").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "F[C@](Cl)(Br)I")
    }

    #[test]
    fn trihalomethane_stereocentric() {
        let graph = from_tree(read("[C@H](F)(Cl)Br").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "[C@H](F)(Cl)Br")
    }

    #[test]
    fn trihalomethane_hydrate_stereocentric() {
        let graph = from_tree(read("O.[C@H](F)(Cl)Br").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "O.[C@H](F)(Cl)Br")
    }

    #[test]
    fn trihalomethane_non_stereocentric() {
        let graph = from_tree(read("F[C@H](Cl)Br").unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), "F[C@H](Cl)Br")
    }
}