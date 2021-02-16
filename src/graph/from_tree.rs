use std::collections::HashMap;

use crate::{ tree, parts };
use super::{ Atom, Bond };

/// Returns a graph-like (adjacency) representation from the corresponding
/// tree. This is useful when analysis requires access to local atomic
/// information.
/// 
/// ```
/// use purr::read::{ read, Error };
/// use purr::graph::{ Atom, Bond, from_tree };
/// use purr::parts::{ AtomKind, Aliphatic, BondKind };
///
/// fn main() -> Result<(), Error> {
///     let root = read("C=*")?.root;
///     let graph = from_tree(root);
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
pub fn from_tree(root: tree::Atom) -> Vec<Atom> {
    let mut stack = Vec::new();
    let mut out = Vec::new();
    let mut opens = HashMap::new();

    out.push(Node::new(root.kind));

    for link in root.links.into_iter().rev() {
        stack.push((0, link))
    }

    while let Some((sid, link)) = stack.pop() {
        add_link(sid, link, &mut stack, &mut opens, &mut out)
    }

    let mut result = Vec::new();

    for node in out {
        result.push(Atom {
            kind: node.kind,
            bonds: node.edges.into_iter().map(|edge| {
                Bond::new(edge.kind, match edge.target {
                    Target::Id(tid) => tid,
                    Target::Rnum(_) => unimplemented!("TODO: unmatched RNUM")
                })
            }).collect::<Vec<_>>()
        })
    }

    result
}

fn add_link(
    sid: usize,
    link: tree::Link,
    stack: &mut Vec<(usize, tree::Link)>,
    opens: &mut HashMap<tree::Rnum, usize>,
    out: &mut Vec<Node>
) {
    let links = match link {
        tree::Link::Bond { kind: bond_kind, target } => {
            match target {
                tree::Target::Atom(target) => {
                    extend(sid, target.kind, bond_kind, out);

                    target.links
                },
                tree::Target::Join(rnum) => {
                    join(sid, bond_kind, rnum, opens, out);
                    
                    return
                }
            }
        },
        tree::Link::Split(target) => {
            out.push(Node::new(target.kind));

            target.links
        }
    };

    let tid = out.len() - 1;

    for link in links.into_iter().rev() {
        stack.push((tid, link))
    }
}

fn extend(
    sid: usize,
    atom: parts::AtomKind,
    input: parts::BondKind,
    out: &mut Vec<Node>,
) {
    let tid = out.len();

    out.push(Node::from(atom, &input, sid));

    let source = &mut out[sid];

    source.edges.push(Edge {
        kind: input,
        target: Target::Id(tid)
    });
}

fn join(
    sid: usize,
    bond_kind: parts::BondKind,
    rnum: tree::Rnum,
    opens: &mut HashMap<tree::Rnum, usize>,
    out: &mut Vec<Node>
) {
    match opens.remove_entry(&rnum) {
        Some((rnum, tid)) => {
            out[sid].edges.push(Edge { kind: bond_kind, target: Target::Id(tid) });
            
            let edge = out[tid].edges.iter_mut().find(|edge| {
                if let Target::Rnum(test) = &edge.target {
                    test == &rnum
                } else {
                    false
                }
            }).expect("edge for rnum");

            std::mem::swap(&mut edge.target, &mut Target::Id(sid))
        },
        None => {
            opens.insert(rnum.clone(), sid);
            out[sid].edges.push(Edge { kind: bond_kind, target: Target::Rnum(rnum) })
        }
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

    fn from(mut kind: parts::AtomKind, input: &parts::BondKind, tid: usize) -> Self {
        let bond = Edge { kind: input.reverse(), target: Target::Id(tid) };

        if let parts::AtomKind::Bracket { configuration, hcount, .. } = &mut kind {
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

enum Target {
    Id(usize),
    Rnum(tree::Rnum)
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
    fn p1() {
        let root = read("*").unwrap().root;

        assert_eq!(from_tree(root), vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ ]
            }
        ])
    }

    #[test]
    fn methane() {
        let root = read("C").unwrap().root;

        assert_eq!(from_tree(root), vec![
            Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                bonds: vec![ ]
            }
        ])
    }

    #[test]
    fn p2() {
        let root = read("**").unwrap().root;

        assert_eq!(from_tree(root), vec![
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
        ])
    }

    #[test]
    fn p1_p1() {
        let root = read("*.*").unwrap().root;

        assert_eq!(from_tree(root), vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ ]
            }
        ])
    }

    #[test]
    fn p3() {
        let root = read("***").unwrap().root;

        assert_eq!(from_tree(root), vec![
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
        ])
    }

    #[test]
    fn p3_branched() {
        let root = read("*(*)*").unwrap().root;

        assert_eq!(from_tree(root), vec![
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
        ])
    }

    #[test]
    fn c3() {
        let root = read("*1**1").unwrap().root;

        assert_eq!(from_tree(root), vec![
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
        ])
    }

    #[test]
    fn c3_left_double() {
        let root = read("*=1**1").unwrap().root;

        assert_eq!(from_tree(root), vec![
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
        ])
    }

    #[test]
    fn c3_right_double() {
        let root = read("*1**=1").unwrap().root;

        assert_eq!(from_tree(root), vec![
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
        ])
    }

    #[test]
    fn c3_left_up_right_down() {
        let root = read("*/1**\\1").unwrap().root;

        assert_eq!(from_tree(root), vec![
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
        ])
    }

    #[test]
    fn p2_up() {
        let root = read("*/*").unwrap().root;

        assert_eq!(from_tree(root), vec![
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
        ])
    }

    #[test]
    fn atom_configuration_hydrogen_stereocentric() {
        let root = read("[C@H](F)(Cl)Br").unwrap().root;

        assert_eq!(from_tree(root)[0], Atom {
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
        let root = read("C[C@H](F)Cl").unwrap().root;

        assert_eq!(from_tree(root)[1], Atom {
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
        let root = read("*12***1**2").unwrap().root;

        assert_eq!(from_tree(root), vec![
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
        ])
    }
}