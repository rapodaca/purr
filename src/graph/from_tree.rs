use std::collections::HashMap;
use std::collections::hash_map::Entry;

use crate::{ tree, parts };
use super::{ Atom, Bond, reconcile_bonds::reconcile_bonds, Error };

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
///     let graph = from_tree(root).expect("semantic error");
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
    let mut out = Vec::new();
    let mut opens: HashMap<tree::Rnum, Open> = HashMap::new();

    out.push(Atom::new(root.kind));

    for link in root.links.into_iter().rev() {
        stack.push((0, link))
    }

    while let Some((sid, link)) = stack.pop() {
        add_link(sid, link, &mut opens, &mut stack, &mut out)?
    }

    Ok(out)
}

fn add_link(
    sid: usize,
    link: tree::Link,
    opens: &mut HashMap<tree::Rnum, Open>,
    stack: &mut Vec<(usize, tree::Link)>,
    out: &mut Vec<Atom>
) -> Result<(), Error> {
    let links = match link {
        tree::Link::Bond { kind: bond_kind, target } => {
            match target {
                tree::Target::Atom(target) => {
                    extend(sid, target.kind, bond_kind, out);

                    target.links
                },
                tree::Target::Join(rnum) => {
                    join(sid, bond_kind, rnum, opens, out)?;
                    
                    return Ok(())
                }
            }
        },
        tree::Link::Split(target) => {
            out.push(Atom::new(target.kind));

            target.links
        }
    };

    let tid = out.len() - 1;

    for link in links.into_iter().rev() {
        stack.push((tid, link))
    }

    Ok(())
}

fn create_atom(
    mut kind: parts::AtomKind,
    input: &parts::BondKind,
    tid: usize
) -> Atom {
    let bond = Bond { kind: input.reverse(), tid };

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

    let result = Atom { kind, bonds: vec![ bond ] };

    result
}

fn extend(
    sid: usize,
    atom: parts::AtomKind,
    input: parts::BondKind,
    out: &mut Vec<Atom>,
) {
    let tid = out.len();

    out.push(create_atom(atom, &input, sid));

    let source = &mut out[sid];

    source.bonds.push(Bond {
        kind: input,
        tid: tid
    });
}

fn join(
    sid: usize,
    bond_kind: parts::BondKind,
    rnum: tree::Rnum,
    opens: &mut HashMap<tree::Rnum, Open>,
    out: &mut Vec<Atom>
) -> Result<(), Error> {
    match opens.entry(rnum) {
        Entry::Occupied(occupied) => {
            let open = occupied.remove();
            
            let (forward, reverse) = match reconcile_bonds(
                bond_kind, open.kind
            ) {
                Some((foreward, reverse)) => (
                    Bond::new(foreward, open.sid),
                    Bond::new(reverse, sid)
                ),
                None => return Err(
                    Error::IncompatibleJoin(open.sid, sid)
                )
            };

            out[sid].bonds.push(forward);
            out[open.sid].bonds.insert(open.index, reverse);
        },
        Entry::Vacant(vacant) => {
            vacant.insert(Open {
                sid: sid,
                kind: bond_kind,
                index: out[sid].bonds.len()
            });
        }
    }

    Ok(())
}

struct Open {
    sid: usize,
    index: usize,
    kind: parts::BondKind
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
    fn incompatible_join() {
        let root = read("*-1**=1").unwrap().root;

        assert_eq!(from_tree(root), Err(Error::IncompatibleJoin(0, 2)))
    }

    #[test]
    fn p1() {
        let root = read("*").unwrap().root;

        assert_eq!(from_tree(root), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ ]
            }
        ]))
    }

    #[test]
    fn methane() {
        let root = read("C").unwrap().root;

        assert_eq!(from_tree(root), Ok(vec![
            Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                bonds: vec![ ]
            }
        ]))
    }

    #[test]
    fn p2() {
        let root = read("**").unwrap().root;

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
        let root = read("*.*").unwrap().root;

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
        let root = read("***").unwrap().root;

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
        let root = read("*(*)*").unwrap().root;

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
        let root = read("*1**1").unwrap().root;

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
        let root = read("*=1**1").unwrap().root;

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
                    Bond::new(BondKind::Double, 0)
                ]
            }
        ]))
    }

    #[test]
    fn c3_right_double() {
        let root = read("*1**=1").unwrap().root;

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
                    Bond::new(BondKind::Double, 0)
                ]
            }
        ]))
    }

    #[test]
    fn c3_left_up_right_down() {
        let root = read("*/1**\\1").unwrap().root;

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
        let root = read("*/*").unwrap().root;

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
        let root = read("[C@H](F)(Cl)Br").unwrap().root;

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
        let root = read("C[C@H](F)Cl").unwrap().root;

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
}