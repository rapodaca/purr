use std::collections::HashMap;
use std::collections::hash_map::Entry;

use crate::tree;
use crate::parts::{ AtomKind, BondKind };
use super::{ Atom, Bond, reconcile_bonds, Error };

pub fn from_tree(root: tree::Atom) -> Result<Vec<Atom>, Error> {
    let mut result = Vec::new();
    let mut opens = HashMap::new();

    walk(None, root, &mut result, &mut opens)?;

    Ok(result)
}

fn walk(
    input: Option<Bond>,
    root: tree::Atom,
    result: &mut Vec<Atom>,
    opens: &mut HashMap<u16, Open>
) -> Result<(), Error> {
    let id = result.len();
    let negate = input.is_some();
    let mut atom = Atom {
        kind: root.kind,
        bonds: match input {
            Some(input) => vec![ input ],
            None => vec![ ]
        }
    };

    if negate {
        match &mut atom.kind {
            AtomKind::Bracket { parity, hcount, .. } => {
                match parity {
                    Some(parity) => {
                        if hcount.unwrap_or_default() > 0 {
                            std::mem::swap(parity, &mut parity.negate())
                        }
                    },
                    None => ()
                }
            }
            _ => ()
        }
    }

    result.push(atom);

    for link in root.links {
        let tid = result.len();

        match link {
            tree::Link::Bond { kind, target } => match target {
                tree::Target::Atom(target) => {
                    let reverse = kind.reverse();

                    result[id].bonds.push(Bond::new(kind, tid));
                    walk(Some(Bond::new(reverse, id)), target, result, opens)?
                },
                tree::Target::Join(rnum) => match opens.entry(rnum) {
                    Entry::Occupied(occupied) => {
                        let open = occupied.remove();

                        let (forward, reverse) = match reconcile_bonds(
                            kind, open.kind
                        ) {
                            Some((forward, reverse)) => (
                                Bond::new(forward, open.sid),
                                Bond::new(reverse, id)
                            ),
                            None => return Err(
                                Error::IncompatibleJoin(open.sid, id)
                            )
                        };

                        result[id].bonds.push(forward);
                        result[open.sid].bonds.insert(open.index, reverse)
                    },
                    Entry::Vacant(vacant) => {
                        vacant.insert(Open {
                            sid: id,
                            kind: kind,
                            index: result[id].bonds.len()
                        });
                    }
                }
            },
            tree::Link::Split(target) => {
                walk(None, target, result, opens)?
            }
        }
    }

    Ok(())
}

struct Open {
    sid: usize,
    index: usize,
    kind: BondKind
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::read::read;
    use crate::parts::{ AtomKind, Aliphatic, BracketSymbol, Element, Parity };
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
    fn p1_p1() {
        let root = read("*.*").unwrap().root;

        assert_eq!(from_tree(root), Ok(vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![

                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![

                ]
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
    fn atom_parity_head_hydrogen() {
        let root = read("[C@H](F)(Cl)Br").unwrap().root;

        assert_eq!(from_tree(root).unwrap()[0], Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Element(Element::C),
                hcount: Some(1),
                charge: None,
                parity: Some(Parity::Counterclockwise),
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
    fn atom_parity_tail() {
        let root = read("F[C@](C)(Cl)Br").unwrap().root;

        assert_eq!(from_tree(root).unwrap()[1], Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Element(Element::C),
                hcount: None,
                charge: None,
                parity: Some(Parity::Counterclockwise),
                map: None
            },
            bonds: vec![
                Bond::new(BondKind::Elided, 0),
                Bond::new(BondKind::Elided, 2),
                Bond::new(BondKind::Elided, 3),
                Bond::new(BondKind::Elided, 4)
            ]
        })
    }

    #[test]
    fn atom_parity_tail_hydrogen() {
        let root = read("F[C@H](Cl)Br").unwrap().root;

        assert_eq!(from_tree(root).unwrap()[1], Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Element(Element::C),
                hcount: Some(1),
                charge: None,
                parity: Some(Parity::Clockwise),
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