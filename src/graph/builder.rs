use std::collections::{ HashMap, hash_map::Entry };

use crate::walk::Follower;
use crate::feature::{ AtomKind, BondKind, Rnum };
use super::{ Atom, Bond, Error, reconcile };

/// A `Follower` that builds an adjacency list SMILES representation.
/// 
/// ```
/// use purr::walk::Follower;
/// use purr::graph::{ Atom, Bond, Builder };
/// use purr::feature::{ AtomKind, BondKind };
/// 
/// fn main() {
///     let mut builder = Builder::new();
/// 
///     builder.root(AtomKind::Star);
///     builder.extend(BondKind::Double, AtomKind::Star);
/// 
///     assert_eq!(builder.build(), Ok(vec![
///         Atom {
///             kind: AtomKind::Star,
///             bonds: vec![
///                 Bond::new(BondKind::Double, 1)
///             ]
///         },
///         Atom {
///             kind: AtomKind::Star,
///             bonds: vec![
///                 Bond::new(BondKind::Double, 0)
///             ]
///         }
///     ]))
/// }
/// ```
#[derive(Debug,PartialEq)]
pub struct Builder {
    stack: Vec<usize>,
    graph: Vec<Node>,
    opens: HashMap<Rnum, usize>,
    errors: Vec<Error>,
    rid: usize
}

impl Builder {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            graph: Vec::new(),
            opens: HashMap::new(),
            errors: Vec::new(),
            rid: 0
        }
    }

    /// Builds the representation created by using the `Follower` trait
    /// methods.
    pub fn build(self) -> Result<Vec<Atom>, Error> {
        if let Some(error) = self.errors.into_iter().next() {
            return Err(error)
        }

        let mut result = Vec::new();
        
        for node in self.graph {
            let mut bonds = Vec::new();

            for edge in node.edges {
                match edge.target {
                    Target::Id(tid) => bonds.push(Bond::new(edge.kind, tid)),
                    Target::Rnum(rid, _, _) => return Err(Error::Rnum(rid))
                }
            }

            result.push(Atom {
                kind: node.kind,
                bonds
            })
        }

        Ok(result)
    }
}

impl Follower for Builder {
    fn root(&mut self, kind: AtomKind) {
        self.stack.push(self.graph.len());
        self.graph.push(Node::parent(kind))
    }

    fn extend(&mut self, bond_kind: BondKind, mut atom_kind: AtomKind) {
        let sid = *self.stack.last().expect("last on stack");
        let tid = self.graph.len();
        let reverse = Edge::new(bond_kind.reverse(), Target::Id(sid));
        let forward = Edge::new(bond_kind, Target::Id(tid));

        atom_kind.invert_configuration();

        self.stack.push(self.graph.len());
        self.graph.push(Node::child(reverse, atom_kind));
        self.graph[sid].edges.push(forward)
    }

    fn join(&mut self, bond_kind: BondKind, rnum: Rnum) {
        match self.opens.entry(rnum) {
            Entry::Occupied(occupied) => {
                let sid = *self.stack.last().expect("last on stack");
                let (rnum, tid) = occupied.remove_entry();
                let edge = self.graph[tid].edges.iter_mut().find(|edge| {
                    if let Target::Rnum(_, _, test) = &edge.target {
                        test == &rnum
                    } else {
                        false
                    }
                }).expect("edge for rnum");

                match reconcile(edge.kind.clone(), bond_kind) {
                    Some((left, right)) => {
                        edge.target = Target::Id(sid);
                        edge.kind = left;

                        self.graph[sid].add_edge(right, Target::Id(tid))
                    },
                    None => self.errors.push(Error::Join(sid, tid))
                }
            },
            Entry::Vacant(vacant) => {
                let sid = *self.stack.last().expect("last on stack");
                let rnum = vacant.key().clone();

                vacant.insert(sid);
                self.graph[sid].add_edge(bond_kind, Target::Rnum(self.rid, sid, rnum));
            }
        }

        self.rid += 1
    }

    fn pop(&mut self, depth: usize) {
        for _ in 0..depth {
            self.stack.pop();
        }
    }
}

#[derive(Debug,PartialEq)]
struct Node {
    kind: AtomKind,
    edges: Vec<Edge>
}

impl Node {
    fn parent(kind: AtomKind) -> Self {
        Self {
            kind,
            edges: Vec::new()
        }
    }

    fn child(input: Edge, kind: AtomKind) -> Self {
        Self {
            kind,
            edges: vec![ input ]
        }
    }

    fn add_edge(&mut self, kind: BondKind, target: Target) {
        self.edges.push(Edge::new(kind, target))
    }
}

#[derive(Debug,PartialEq)]
struct Edge {
    kind: BondKind,
    target: Target
}

impl Edge {
    fn new(kind: BondKind, target: Target) -> Self {
        Self {
            kind,
            target
        }
    }
}

#[derive(Debug,PartialEq)]
enum Target {
    Id(usize),
    // rid, sid, rnum
    Rnum(usize, usize, Rnum)
}

#[cfg(test)]
mod errors {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn join_incompatible() {
        let mut builder = Builder::new();

        builder.root(AtomKind::Star);
        builder.join(BondKind::Up, Rnum::R1);
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.join(BondKind::Up, Rnum::R1);

        assert_eq!(builder.build(), Err(Error::Join(2, 0)))
    }

    #[test]
    fn join_unbalanced() {
        let mut builder = Builder::new();

        builder.root(AtomKind::Star);
        builder.join(BondKind::Elided, Rnum::R1);
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.join(BondKind::Elided, Rnum::R1);
        builder.join(BondKind::Elided, Rnum::R2);

        assert_eq!(builder.build(), Err(Error::Rnum(2)))
    }
}

#[cfg(test)]
mod build {
    use pretty_assertions::assert_eq;
    use crate::feature::{ BracketSymbol, VirtualHydrogen, Configuration };
    use super::*;

    #[test]
    fn p1() {
        let mut builder = Builder::new();

        builder.root(AtomKind::Star);

        assert_eq!(builder.build(), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ ]
            }
        ]))
    }

    #[test]
    fn p2() {
        let mut builder = Builder::new();

        builder.root(AtomKind::Star);
        builder.extend(BondKind::Elided, AtomKind::Star);

        assert_eq!(builder.build(), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 1)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 0)
                ]
            }
        ]))
    }

    #[test]
    fn p3() {
        let mut builder = Builder::new();

        builder.root(AtomKind::Star);
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.extend(BondKind::Single, AtomKind::Star);

        assert_eq!(builder.build(), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 1)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 0),
                    Bond::new(BondKind::Single, 2)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Single, 1)
                ]
            }
        ]))
    }

    #[test]
    fn p3_branched() {
        let mut builder = Builder::new();

        builder.root(AtomKind::Star);
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.pop(1);
        builder.extend(BondKind::Single, AtomKind::Star);

        assert_eq!(builder.build(), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 1),
                    Bond::new(BondKind::Single, 2)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 0)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Single, 0)
                ]
            }
        ]))
    }

    #[test]
    fn c3_elided_elided() {
        let mut builder = Builder::new();

        builder.root(AtomKind::Star);
        builder.join(BondKind::Elided, Rnum::R1);
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.join(BondKind::Elided, Rnum::R1);

        assert_eq!(builder.build(), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 2),
                    Bond::new(BondKind::Elided, 1)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 0),
                    Bond::new(BondKind::Elided, 2)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 1),
                    Bond::new(BondKind::Elided, 0)
                ]
            }
        ]))
    }

    #[test]
    fn c3_single_elided() {
        let mut builder = Builder::new();

        builder.root(AtomKind::Star);
        builder.join(BondKind::Single, Rnum::R1);
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.join(BondKind::Elided, Rnum::R1);

        assert_eq!(builder.build(), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Single, 2),
                    Bond::new(BondKind::Elided, 1)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 0),
                    Bond::new(BondKind::Elided, 2)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 1),
                    Bond::new(BondKind::Single, 0)
                ]
            }
        ]))
    }

    #[test]
    fn tetrahedral_root() {
        let mut builder = Builder::new();

        builder.root(AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            configuration: Some(Configuration::TH1),
            hcount: None,
            charge: None,
            map: None
        });
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.pop(1);
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.pop(1);
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.pop(1);
        builder.extend(BondKind::Elided, AtomKind::Star);

        assert_eq!(builder.build(), Ok(vec![
            Atom {
                kind: AtomKind::Bracket {
                    isotope: None,
                    symbol: BracketSymbol::Star,
                    configuration: Some(Configuration::TH1),
                    hcount: None,
                    charge: None,
                    map: None
                },
                bonds: vec![
                    Bond::new(BondKind::Elided, 1),
                    Bond::new(BondKind::Elided, 2),
                    Bond::new(BondKind::Elided, 3),
                    Bond::new(BondKind::Elided, 4)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 0) ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 0) ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 0) ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 0) ]
            }
        ]))
    }

    #[test]
    fn tetrahedral_child() {
        let mut builder = Builder::new();

        builder.root(AtomKind::Star);
        builder.extend(BondKind::Elided, AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            configuration: Some(Configuration::TH1),
            hcount: None,
            charge: None,
            map: None
        });
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.pop(1);
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.pop(1);
        builder.extend(BondKind::Elided, AtomKind::Star);

        assert_eq!(builder.build(), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            },
            Atom {
                kind: AtomKind::Bracket {
                    isotope: None,
                    symbol: BracketSymbol::Star,
                    configuration: Some(Configuration::TH1),
                    hcount: None,
                    charge: None,
                    map: None
                },
                bonds: vec![
                    Bond::new(BondKind::Elided, 0),
                    Bond::new(BondKind::Elided, 2),
                    Bond::new(BondKind::Elided, 3),
                    Bond::new(BondKind::Elided, 4)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            }
        ]))
    }

    #[test]
    fn tetrahedral_child_hydrogen() {
        let mut builder = Builder::new();

        builder.root(AtomKind::Star);
        builder.extend(BondKind::Elided, AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            configuration: Some(Configuration::TH1),
            hcount: Some(VirtualHydrogen::H1),
            charge: None,
            map: None
        });
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.pop(1);
        builder.extend(BondKind::Elided, AtomKind::Star);
        builder.pop(1);
        builder.extend(BondKind::Elided, AtomKind::Star);

        assert_eq!(builder.build(), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            },
            Atom {
                kind: AtomKind::Bracket {
                    isotope: None,
                    symbol: BracketSymbol::Star,
                    configuration: Some(Configuration::TH2),
                    hcount: Some(VirtualHydrogen::H1),
                    charge: None,
                    map: None
                },
                bonds: vec![
                    Bond::new(BondKind::Elided, 0),
                    Bond::new(BondKind::Elided, 2),
                    Bond::new(BondKind::Elided, 3),
                    Bond::new(BondKind::Elided, 4)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            }
        ]))
    }
}