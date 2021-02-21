use std::collections::HashMap;

use crate::{ tree, parts };
use super::{ Atom, Bond, Trace, Error };

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
///     let graph = from_tree(root, None)?;
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
pub fn from_tree(
    root: tree::Atom, trace: Option<&mut Trace>
) -> Result<Vec<Atom>, Error> {
    let mut stack = Vec::new();
    let mut builder = GraphBuilder {
        opens: HashMap::new(),
        out: Vec::new(),
        rid: 0,
        trace
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

struct GraphBuilder<'a> {
    opens: HashMap<tree::Rnum, usize>,
    out: Vec<Node>,
    trace: Option<&'a mut Trace>,
    rid: usize
}

impl<'a> GraphBuilder<'a> {
    fn add_root(&mut self, kind: parts::AtomKind) {
        self.out.push(Node::new(kind))
    }

    fn extend(
        &mut self, sid: usize, input: parts::BondKind, atom: parts::AtomKind
    ) {
        let tid = self.out.len();

        self.out.push(Node::from(atom, &input, sid));
        self.out[sid].edges.push(Edge::new(input, Target::Id(tid)));

        if let Some(trace) = self.trace.as_mut() {
            trace.bond(sid, tid)
        }
    }

    fn join(
        &mut self, sid: usize, bond_kind: parts::BondKind, rnum: tree::Rnum
    ) {
        if let Some(trace) = self.trace.as_mut() {
            trace.join(sid, rnum.clone())
        }

        match self.opens.remove_entry(&rnum) {
            Some((rnum, tid)) => {
                self.out[sid].edges.push(Edge::new(bond_kind, Target::Id(tid)));
                
                let edge = self.out[tid].edges.iter_mut().find(|edge| {
                    if let Target::Rnum(_, _, test) = &edge.target {
                        test == &rnum
                    } else {
                        false
                    }
                }).expect("edge for rnum");
    
                std::mem::swap(&mut edge.target, &mut Target::Id(sid))
            },
            None => {
                self.opens.insert(rnum.clone(), sid);
                self.out[sid].edges.push(
                    Edge::new(bond_kind, Target::Rnum(self.rid, sid, rnum))
                )
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
                    Target::Rnum(rid, _, _) =>
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
    // rid, sid, rnum
    Rnum(usize, usize, tree::Rnum)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::read::{ read, Trace as TreeTrace };
    use crate::parts::{
        AtomKind, BondKind, Aliphatic, BracketSymbol, Element, Configuration,
        VirtualHydrogen
    };
    use super::*;

    #[test]
    fn unbalanced_rnum_single() {
        let root = read("*1", None).unwrap();

        assert_eq!(from_tree(root, None), Err(Error::UnbalancedRnum(0)))
    }

    #[test]
    fn unbalanced_rnum_first() {
        let root = read("*1**1*2**3**", None).unwrap();

        assert_eq!(from_tree(root, None), Err(Error::UnbalancedRnum(2)))
    }

    #[test]
    fn unbalanced_rnum_last() {
        let root = read("**1**1*2**", None).unwrap();

        assert_eq!(from_tree(root, None), Err(Error::UnbalancedRnum(2)))
    }

    #[test]
    fn p1() {
        let root = read("*", None).unwrap();

        assert_eq!(from_tree(root, None), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ ]
            }
        ]))
    }

    #[test]
    fn p1_trace() {
        let mut trace = TreeTrace::new();
        let root = read("*", Some(&mut trace)).unwrap();
        let mut trace = Trace::new(trace);

        from_tree(root, Some(&mut trace)).unwrap();

        assert_eq!(trace.atom_cursor(0), Some(0..1))
    }

    #[test]
    fn methane() {
        let root = read("C", None).unwrap();

        assert_eq!(from_tree(root, None), Ok(vec![
            Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                bonds: vec![ ]
            }
        ]))
    }

    #[test]
    fn p2() {
        let root = read("**", None).unwrap();

        assert_eq!(from_tree(root, None), Ok(vec![
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
    fn p2_trace() {
        let mut trace = TreeTrace::new();
        let root = read("**", Some(&mut trace)).unwrap();
        let mut trace = Trace::new(trace);

        from_tree(root, Some(&mut trace)).unwrap();

        assert_eq!(trace.bond_cursor(0, 1), Some(1))
    }

    #[test]
    fn p1_p1() {
        let root = read("*.*", None).unwrap();

        assert_eq!(from_tree(root, None), Ok(vec![
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
    fn p1_p1_trace() {
        let mut trace = TreeTrace::new();
        let root = read("*.*", Some(&mut trace)).unwrap();
        let mut trace = Trace::new(trace);

        from_tree(root, Some(&mut trace)).unwrap();

        assert_eq!(trace.bond_cursor(0, 1), None)
    }

    #[test]
    fn p3() {
        let root = read("***", None).unwrap();

        assert_eq!(from_tree(root, None), Ok(vec![
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

        assert_eq!(from_tree(root, None), Ok(vec![
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
    fn p3_branched_trace() {
        let mut trace = TreeTrace::new();
        let root = read("*(*)*", Some(&mut trace)).unwrap();
        let mut trace = Trace::new(trace);

        from_tree(root, Some(&mut trace)).unwrap();

        assert_eq!(trace.atom_cursor(0), Some(0..1));
        assert_eq!(trace.atom_cursor(1), Some(2..3));
        assert_eq!(trace.atom_cursor(2), Some(4..5));
        assert_eq!(trace.bond_cursor(0, 1), Some(2));
        assert_eq!(trace.bond_cursor(0, 2), Some(4))
    }

    #[test]
    fn c3() {
        let root = read("*1**1", None).unwrap();

        assert_eq!(from_tree(root, None), Ok(vec![
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
    fn c3_trace() {
        let mut trace = TreeTrace::new();
        let root = read("*1**1", Some(&mut trace)).unwrap();
        let mut trace = Trace::new(trace);

        from_tree(root, Some(&mut trace)).unwrap();

        assert_eq!(trace.bond_cursor(0, 2), Some(1));
        assert_eq!(trace.bond_cursor(2, 0), Some(4))
    }

    #[test]
    fn c3_left_double() {
        let root = read("*=1**1", None).unwrap();

        assert_eq!(from_tree(root, None), Ok(vec![
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

        assert_eq!(from_tree(root, None), Ok(vec![
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

        assert_eq!(from_tree(root, None), Ok(vec![
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
    fn diamond_trace() {
        let mut trace = TreeTrace::new();
        let root = read("*12**1*2", Some(&mut trace)).unwrap();
        let mut trace = Trace::new(trace);

        from_tree(root, Some(&mut trace)).unwrap();

        assert_eq!(trace.bond_cursor(0, 3), Some(2));
        assert_eq!(trace.bond_cursor(3, 0), Some(7))
    }

    #[test]
    fn p2_up() {
        let root = read("*/*", None).unwrap();

        assert_eq!(from_tree(root, None), Ok(vec![
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

        assert_eq!(from_tree(root, None).unwrap()[0], Atom {
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

        assert_eq!(from_tree(root, None).unwrap()[1], Atom {
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

        assert_eq!(from_tree(root, None), Ok(vec![
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