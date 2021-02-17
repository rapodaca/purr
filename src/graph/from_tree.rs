use std::collections::HashMap;

use crate::{ tree, parts };
use super::{ Atom, Bond, Error };

/// Returns a graph-like (adjacency) representation from the corresponding
/// tree. This is useful when analysis requires access to local atomic
/// information.
/// 
/// ```
/// use purr::read::{ read };
/// use purr::graph::{ Atom, Bond, from_tree, Error };
/// use purr::parts::{ AtomKind, Aliphatic, BondKind };
///
/// fn main() -> Result<(), Error> {
///     let root = read("C=*", None).unwrap();
///     let graph = from_tree(root)?;
/// 
///     assert_eq!(graph, vec![
///         Atom {
///             kind: AtomKind::Aliphatic(Aliphatic::C),
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
///     ]);
///
///     assert_eq!(graph[0].is_aromatic(), false);
///     assert_eq!(graph[0].subvalence(), 2);
/// 
///     Ok(())
/// }
/// ```
pub fn from_tree(root: tree::Atom) -> Result<Vec<Atom>, Error> {
    let mut stack = Vec::new();
    let mut builder = GraphBuilder {
        opens: HashMap::new(),
        out: Vec::new(),
        rid: 0
    };

    builder.add_root(root.kind);

    for link in root.links.into_iter().rev() {
        stack.push((0, link))
    }

    while let Some((sid, link)) = stack.pop() {
        add_link(sid, link, &mut stack, &mut builder)
    }

    builder.build()
}

fn add_link(
    sid: usize,
    link: tree::Link,
    stack: &mut Vec<(usize, tree::Link)>,
    builder: &mut GraphBuilder
) {
    let links = match link {
        tree::Link::Bond { kind: bond_kind, target } => match target {
            tree::Target::Atom(target) => {
                builder.extend(sid, bond_kind, target.kind);

                target.links
            },
            tree::Target::Join(rnum) => {
                builder.join(sid, bond_kind, rnum);
                
                return
            }
        },
        tree::Link::Split(target) => {
            builder.add_root(target.kind);

            target.links
        }
    };

    let tid = builder.id();

    for link in links.into_iter().rev() {
        stack.push((tid, link))
    }
}

struct GraphBuilder {
    opens: HashMap<tree::Rnum, usize>,
    out: Vec<Node>,
    rid: usize
}

impl GraphBuilder {
    fn add_root(&mut self, kind: parts::AtomKind) {
        self.out.push(Node::new(kind))
    }

    fn extend(
        &mut self, sid: usize, input: parts::BondKind, atom: parts::AtomKind
    ) {
        let tid = self.out.len();

        self.out.push(Node::from(atom, &input, sid));
    
        let source = &mut self.out[sid];
    
        source.edges.push(Edge {
            kind: input,
            target: Target::Id(tid)
        });
    }

    fn add_edge(
        &mut self, sid: usize, kind: parts::BondKind, target: Target
    ) {
        self.out[sid].edges.push(Edge::new(kind, target))
    }

    fn join(
        &mut self, sid: usize, bond_kind: parts::BondKind, rnum: tree::Rnum
    ) {
        match self.opens.remove_entry(&rnum) {
            Some((rnum, tid)) => {
                self.add_edge(sid, bond_kind, Target::Id(tid));
                
                let edge = self.out[tid].edges.iter_mut().find(|edge| {
                    if let Target::Rnum(_, test) = &edge.target {
                        test == &rnum
                    } else {
                        false
                    }
                }).expect("edge for rnum");
    
                std::mem::swap(&mut edge.target, &mut Target::Id(sid))
            },
            None => {
                self.opens.insert(rnum.clone(), sid);
                self.add_edge(sid, bond_kind, Target::Rnum(self.rid, rnum))
            }
        }
    
        self.rid += 1
    }

    fn id(&self) -> usize {
        self.out.len() - 1
    }

    fn build(self) -> Result<Vec<Atom>, Error> {
        let mut result = Vec::new();

        for node in self.out {
            let mut bonds = Vec::new();

            for edge in node.edges {
                match edge.target {
                    Target::Id(tid) => bonds.push(Bond::new(edge.kind, tid)),
                    Target::Rnum(rid, _) =>
                        return Err(Error::UnbalancedRnum(rid))
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

struct Node {
    kind: parts::AtomKind,
    edges: Vec<Edge>
}

impl Node {
    fn new(kind: parts::AtomKind) -> Self {
        Self {
            kind,
            edges: vec![ ]
        }
    }

    fn from(
        mut kind: parts::AtomKind, input: &parts::BondKind, tid: usize
    ) -> Self {
        let bond = Edge { kind: input.reverse(), target: Target::Id(tid) };

        if let parts::AtomKind::Bracket {
            configuration, hcount, ..
        } = &mut kind {
            if let Some(configuration) = configuration {
                match hcount {
                    Some(hcount) => if !hcount.is_zero() {
                        if configuration == &parts::Configuration::TH2 {
                            std::mem::swap(
                                configuration, &mut parts::Configuration::TH1
                            )
                        } else if configuration == &parts::Configuration::TH1 {
                            std::mem::swap(
                                configuration, &mut parts::Configuration::TH2
                            )
                        }
                    },
                    None => ()
                }
            }
        }

        Self {
            kind,
            edges: vec![ bond ]
        }
    }
}

struct Edge {
    kind: parts::BondKind,
    target: Target
}

impl Edge {
    fn new(kind: parts::BondKind, target: Target) -> Self {
        Self {
            kind,
            target
        }
    }
}

enum Target {
    Id(usize),
    Rnum(usize, tree::Rnum)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::read::read;
    use crate::parts::{
        AtomKind, BondKind, Aliphatic, BracketSymbol, Element, Configuration,
        VirtualHydrogen
    };
    use super::*;

    #[test]
    fn unbalanced_rnum_single() {
        let root = read("*1", None).unwrap();

        assert_eq!(from_tree(root), Err(Error::UnbalancedRnum(0)))
    }

    #[test]
    fn unbalanced_rnum_first() {
        let root = read("*1**1*2**3**", None).unwrap();

        assert_eq!(from_tree(root), Err(Error::UnbalancedRnum(2)))
    }

    #[test]
    fn unbalanced_rnum_last() {
        let root = read("**1**1*2**", None).unwrap();

        assert_eq!(from_tree(root), Err(Error::UnbalancedRnum(2)))
    }

    #[test]
    fn p1() {
        let root = read("*", None).unwrap();

        assert_eq!(from_tree(root), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ ]
            }
        ]))
    }

    #[test]
    fn methane() {
        let root = read("C", None).unwrap();

        assert_eq!(from_tree(root), Ok(vec![
            Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                bonds: vec![ ]
            }
        ]))
    }

    #[test]
    fn p2() {
        let root = read("**", None).unwrap();

        assert_eq!(from_tree(root), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(parts::BondKind::Elided, 1)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(parts::BondKind::Elided, 0)
                ]
            }
        ]))
    }

    #[test]
    fn p1_p1() {
        let root = read("*.*", None).unwrap();

        assert_eq!(from_tree(root), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ ]
            }
        ]))
    }

    #[test]
    fn p3() {
        let root = read("***", None).unwrap();

        assert_eq!(from_tree(root), Ok(vec![
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
                    Bond::new(BondKind::Elided, 2)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 1)
                ]
            }
        ]))
    }

    #[test]
    fn p3_branched() {
        let root = read("*(*)*", None).unwrap();

        assert_eq!(from_tree(root), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 1),
                    Bond::new(BondKind::Elided, 2)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 0),
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
    fn c3() {
        let root = read("*1**1", None).unwrap();

        assert_eq!(from_tree(root), Ok(vec![
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
    fn c3_left_double() {
        let root = read("*=1**1", None).unwrap();

        assert_eq!(from_tree(root), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Double, 2),
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
    fn c3_right_double() {
        let root = read("*1**=1", None).unwrap();

        assert_eq!(from_tree(root), Ok(vec![
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
                    Bond::new(BondKind::Double, 0)
                ]
            }
        ]))
    }

    #[test]
    fn c3_left_up_right_down() {
        let root = read("*/1**\\1", None).unwrap();

        assert_eq!(from_tree(root), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Up, 2),
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
                    Bond::new(BondKind::Down, 0)
                ]
            }
        ]))
    }

    #[test]
    fn p2_up() {
        let root = read("*/*", None).unwrap();

        assert_eq!(from_tree(root), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Up, 1)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Down, 0)
                ]
            }
        ]))
    }

    #[test]
    fn atom_configuration_hydrogen_stereocentric() {
        let root = read("[C@H](F)(Cl)Br", None).unwrap();

        assert_eq!(from_tree(root).unwrap()[0], Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Element(Element::C),
                hcount: Some(VirtualHydrogen::H1),
                charge: None,
                configuration: Some(Configuration::TH1),
                map: None
            },
            bonds: vec![
                Bond::new(BondKind::Elided, 1),
                Bond::new(BondKind::Elided, 2),
                Bond::new(BondKind::Elided, 3)
            ]
        })
    }

    #[test]
    fn atom_configuration_hydrogen_nonstereocentric() {
        let root = read("C[C@H](F)Cl", None).unwrap();

        assert_eq!(from_tree(root).unwrap()[1], Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Element(Element::C),
                hcount: Some(VirtualHydrogen::H1),
                charge: None,
                configuration: Some(Configuration::TH2),
                map: None
            },
            bonds: vec![
                Bond::new(BondKind::Elided, 0),
                Bond::new(BondKind::Elided, 2),
                Bond::new(BondKind::Elided, 3)
            ]
        })
    }

    #[test]
    fn bicyclo_220() {
        let root = read("*12***1**2", None).unwrap();

        assert_eq!(from_tree(root), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 3),
                    Bond::new(BondKind::Elided, 5),
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
                    Bond::new(BondKind::Elided, 3)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 2),
                    Bond::new(BondKind::Elided, 0),
                    Bond::new(BondKind::Elided, 4)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 3),
                    Bond::new(BondKind::Elided, 5)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 4),
                    Bond::new(BondKind::Elided, 0)
                ]
            }
        ]))
    }
}