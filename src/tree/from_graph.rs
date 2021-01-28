use std::collections::HashMap;

use crate::graph;
use crate::parts::BondKind;
use super::{ Atom, Link, Target, JoinPool, join_pool::Hit };

enum TargetId {
    Id(usize),
    Rnum(u16)
}

pub fn from_graph(graph: Vec<graph::Atom>) -> Atom {
    let mut id_to_atom = HashMap::new();
    let mut id_to_bonds = HashMap::new();
    let mut stack = Vec::new();
    let mut bonds = Vec::new();
    let mut pool = JoinPool::new();

    for (id, atom) in graph.into_iter().enumerate() {
        id_to_atom.insert(id, Atom {
            kind: atom.kind,
            links: vec![ ]
        });
        id_to_bonds.insert(id, atom.bonds);
    }

    for bond in id_to_bonds.remove(&0).expect("root bonds") {
        stack.push((0, bond))
    }

    loop {
        let (sid, bond) = match stack.pop() {
            Some(entry) => entry,
            None => break
        };

        match id_to_bonds.remove(&bond.tid) {
            Some(outs) => {
                for out in outs.into_iter().filter(|out| out.tid != sid) {
                    stack.push((bond.tid, out))
                }

                bonds.push((sid, bond.kind, TargetId::Id(bond.tid)))
            },
            None => {
                let Hit { rnum, closes } = pool.hit(sid, bond.tid);
                let kind = if closes { bond.kind } else { BondKind::Elided };

                bonds.push((sid, kind, TargetId::Rnum(rnum)))
            }
        }
    }

    loop {
        match bonds.pop() {
            Some((sid, kind, target_id)) => {
                match target_id {
                    TargetId::Id(tid) => {
                        let target = id_to_atom.remove(&tid).expect("target");
                        let source = id_to_atom.get_mut(&sid).expect("source");

                        source.links.push(Link::Bond {
                            kind: kind,
                            target: Target::Atom(target)
                        })
                    },
                    TargetId::Rnum(rnum) => {
                        let source = id_to_atom.get_mut(&sid).expect("source");

                        source.links.push(Link::Bond {
                            kind: kind,
                            target: Target::Join(rnum)
                        })
                    }
                }
            },
            None => break id_to_atom.remove(&0).expect("root")
        }
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
    fn trihalogen_methane_s3() {
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
}