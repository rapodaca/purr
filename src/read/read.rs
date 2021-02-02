use crate::tree::{ Atom, Link, Target };
use crate::parts::{ BondKind };
use super::{
    scanner::Scanner,
    Reading,
    Error,
    read_organic::read_organic,
    read_bracket::read_bracket,
    read_bond::read_bond,
    missing_character::missing_character,
    read_rnum::read_rnum
};

/// Reads a SMILES string, returning a Reading, or Error if not successful.
/// Uses the grammar published in [SMILES Formal Grammar Revisited](https://depth-first.com/articles/2020/12/21/smiles-formal-grammar-revisited/).
/// 
/// Branches are read in the order they appear in the string.
/// 
/// ```
/// use purr::read::{ read, Reading, Error };
/// use purr::parts::{ AtomKind, Aromatic };
/// 
/// fn main() -> Result<(), Error> {
///     let Reading { root, trace } = read("c1ccccc1")?;
/// 
///     assert_eq!(root.kind, AtomKind::Aromatic(Aromatic::C));
///     assert_eq!(trace, vec![ 0, 2, 3, 4, 5, 6 ]);
/// 
///     Ok(())
/// }
/// ```
/// 
/// The result type `Reading` is designed to allow semantic errors to be
/// reported in terms of a cursor position in the original string. In the
/// example above, the second atom cursor position
/// is accessed with `trace[1]`, which yields the value `2`. Indexing into
/// the `trace` array is zero-based.
pub fn read(smiles: &str) -> Result<Reading, Error> {
    let mut scanner = Scanner::new(smiles);
    let mut trace = Vec::new();
    let root = match read_smiles(&mut scanner, &mut trace)? {
        Some(root) => root,
        None => if scanner.is_done() {
            return Err(Error::EndOfLine)
        } else {
            return Err(Error::InvalidCharacter(scanner.cursor()))
        }
    };

    if scanner.is_done() {
        Ok(Reading {
            root,
            trace
        })
    } else {
        Err(Error::InvalidCharacter(scanner.cursor()))
    }
}

// <smiles> ::= <atom> <body>*
fn read_smiles(
    scanner: &mut Scanner, trace: &mut Vec<usize>
) -> Result<Option<Atom>, Error> {
    let mut root = match read_atom(scanner, trace)? {
        Some(root) => root,
        None => return Ok(None)
    };

    while let Some(body) = read_body(scanner, trace)? {
        root.links.push(body)
    }

    Ok(Some(root))
}

// <atom> ::= <organic> | <bracket> | <star>
fn read_atom(
    scanner: &mut Scanner, trace: &mut Vec<usize>
) -> Result<Option<Atom>, Error> {
    let cursor = scanner.cursor();
    let result = match read_organic(scanner)? {
        Some(atom) => atom,
        None => match read_bracket(scanner)? {
            Some(atom) => atom,
            None => match scanner.peek() {
                Some('*') => {
                    scanner.pop();

                    Atom::star()
                },
                _ => return Ok(None)
            }
        }
    };

    trace.push(cursor);

    Ok(Some(result))
}

// <body> ::= <branch> | <split> | <union>
fn read_body(
    scanner: &mut Scanner, trace: &mut Vec<usize>
) -> Result<Option<Link>, Error> {
    if let Some(branch) = read_branch(scanner, trace)? {
        return Ok(Some(branch))
    }

    if let Some(split) = read_split(scanner, trace)? {
        return Ok(Some(split))
    }

    read_union(scanner, trace)
}

// <branch> ::= "(" ( <dot> | <bond> )? <smiles> ")"
fn read_branch(
    scanner: &mut Scanner,
    trace: &mut Vec<usize>
) -> Result<Option<Link>, Error> {
    match scanner.peek() {
        Some('(') => {
            scanner.pop();
        },
        _ => return Ok(None)
    }

    let result = match scanner.peek() {
        Some('.') => {
            scanner.pop();

            match read_smiles(scanner, trace)? {
                Some(target) => Link::Split(target),
                None => return Err(missing_character(scanner))
            }
        },
        _ => {
            let kind = read_bond(scanner);

            match read_smiles(scanner, trace)? {
                Some(target) => Link::Bond {
                    kind,
                    target: Target::Atom(target)
                },
                None => return Err(missing_character(scanner))
            }
        }
    };

    match scanner.peek() {
        Some(')') => {
            scanner.pop();

            Ok(Some(result))
        },
        _ => Err(missing_character(scanner))
    }
}

// <split> ::= <dot> <smiles>
fn read_split(
    scanner: &mut Scanner,
    trace: &mut Vec<usize>
) -> Result<Option<Link>, Error> {
    match scanner.peek() {
        Some('.') => {
            scanner.pop();
        },
        _ => return Ok(None)
    }

    match read_smiles(scanner, trace)? {
        Some(smiles) => Ok(Some(Link::Split(smiles))),
        _ => Err(missing_character(scanner))
    }
}

// <union> ::= <bond>? ( <smiles> | <rnum> )
fn read_union(
    scanner: &mut Scanner,
    trace: &mut Vec<usize>
) -> Result<Option<Link>, Error> {
    let bond_kind = read_bond(scanner);
    let target = if let Some(smiles) = read_smiles(scanner, trace)? {
        Target::Atom(smiles)
    } else if let Some(rnum) = read_rnum(scanner)? {
        Target::Join(rnum)
    } else if bond_kind == BondKind::Elided {
        return Ok(None)
    } else {
        return Err(missing_character(scanner))
    };

    Ok(Some(Link::Bond {
        kind: bond_kind,
        target: target
    }))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::parts::{ Element, AtomKind, Aliphatic, BracketSymbol, Aromatic };
    use super::*;

    #[test]
    fn blank() {
        assert_eq!(read(""), Err(Error::EndOfLine))
    }

    #[test]
    fn leading_paren() {
        assert_eq!(read("("), Err(Error::InvalidCharacter(0)))
    }

    #[test]
    fn invalid_tail() {
        assert_eq!(read("C?"), Err(Error::InvalidCharacter(1)))
    }

    #[test]
    fn trailing_bond() {
        assert_eq!(read("*-"), Err(Error::EndOfLine))
    }

    #[test]
    fn trailing_dot() {
        assert_eq!(read("*."), Err(Error::EndOfLine))
    }

    #[test]
    fn cut_percent_single_digit() {
        assert_eq!(read("C%1N"), Err(Error::InvalidCharacter(3)))
    }

    #[test]
    fn open_paren_eol() {
        assert_eq!(read("C("), Err(Error::EndOfLine))
    }

    #[test]
    fn missing_close_paren() {
        assert_eq!(read("C(C"), Err(Error::EndOfLine))
    }

    #[test]
    fn bond_to_invalid() {
        assert_eq!(read("C-X"), Err(Error::InvalidCharacter(2)))
    }

    #[test]
    fn branch_invalid() {
        assert_eq!(read("C(?)C"), Err(Error::InvalidCharacter(2)))
    }

    #[test]
    fn branch_rnum() {
        assert_eq!(read("C(1)C"), Err(Error::InvalidCharacter(2)))
    }

    #[test]
    fn branch_bond_rnum() {
        assert_eq!(read("C(-1)C"), Err(Error::InvalidCharacter(3)))
    }

    #[test]
    fn dot_rnum() {
        assert_eq!(read("C.1"), Err(Error::InvalidCharacter(2)));
    }

    #[test]
    fn star() {
        assert_eq!(read("*"), Ok(Reading {
            root: Atom::star(),
            trace: vec![ 0 ]
        }))
    }

    #[test]
    fn methane_bracket() {
        assert_eq!(read("[CH4]"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Bracket {
                    isotope: None,
                    symbol: BracketSymbol::Element(Element::C),
                    parity: None,
                    hcount: Some(4),
                    charge: None,
                    map: None
                },
                links: vec![ ]
            },
            trace: vec![ 0 ]
        }))
    }

    #[test]
    fn aromatic() {
        assert_eq!(read("c"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aromatic(Aromatic::C),
                links: vec![ ]
            },
            trace: vec![ 0 ]
        }))
    }

    #[test]
    fn atom_rnum_implicit() {
        assert_eq!(read("C1"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![ Link::elided_join(1) ]
            },
            trace: vec![ 0 ]
        }))
    }

    #[test]
    fn implicit_bond() {
        assert_eq!(read(&"CO"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::elided_atom(AtomKind::Aliphatic(Aliphatic::O))
                ]
            },
            trace: vec![ 0, 1 ]
        }))
    }

    #[test]
    fn single_bond() {
        assert_eq!(read("C-O"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::single_atom(AtomKind::Aliphatic(Aliphatic::O))
                ]
            },
            trace: vec![ 0, 2 ]
        }))
    }

    #[test]
    fn double_bond() {
        assert_eq!(read("C=O"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::double_atom(AtomKind::Aliphatic(Aliphatic::O))
                ]
            },
            trace: vec![ 0, 2 ]
        }))
    }

    #[test]
    fn ethane_bracket() {
        assert_eq!(read("C[CH4]"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![ 
                    Link::Bond {
                        kind: BondKind::Elided,
                        target: Target::Atom(Atom {
                            kind: AtomKind::Bracket {
                                isotope: None,
                                symbol: BracketSymbol::Element(Element::C),
                                parity: None,
                                hcount: Some(4),
                                charge: None,
                                map: None
                            },
                            links: vec![ ]
                        })
                    }
                ]
            },
            trace: vec![ 0, 1 ]
        }))
    }

    #[test]
    fn implicit_aromatic() {
        assert_eq!(read(&"cn"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aromatic(Aromatic::C),
                links: vec![
                    Link::elided_atom(AtomKind::Aromatic(Aromatic::N))
                ]
            },
            trace: vec![ 0, 1 ]
        }))
    }

    #[test]
    fn split() {
        assert_eq!(read(&"C.O"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::Split(Atom::aliphatic(Aliphatic::O))
                ]
            },
            trace: vec![ 0, 2 ]
        }))
    }

    #[test]
    fn branch_singleton_elided() {
        assert_eq!(read("C(O)N"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::elided_atom(AtomKind::Aliphatic(Aliphatic::O)),
                    Link::elided_atom(AtomKind::Aliphatic(Aliphatic::N))
                ]
            },
            trace: vec![ 0, 2, 4 ]
        }))
    }

    #[test]
    fn branch_singleton_double() {
        assert_eq!(read("C(=O)N"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::Bond {
                        kind: BondKind::Double,
                        target: Target::Atom(Atom::aliphatic(Aliphatic::O))
                    },
                    Link::elided_atom(AtomKind::Aliphatic(Aliphatic::N))
                ]
            },
            trace: vec![ 0, 3, 5 ]
        }))
    }

    #[test]
    fn branch_chain_single() {
        assert_eq!(read("C(-OC)N"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::Bond {
                        kind: BondKind::Single,
                        target: Target::Atom(Atom {
                            kind: AtomKind::Aliphatic(Aliphatic::O),
                            links: vec![
                                Link::elided_atom(
                                    AtomKind::Aliphatic(Aliphatic::C)
                                )
                            ]
                        })
                    },
                    Link::elided_atom(AtomKind::Aliphatic(Aliphatic::N))
                ]
            },
            trace: vec![ 0, 3, 4, 6 ]
        }))
    }

    #[test]
    fn branch_branch_single() {
        assert_eq!(read("C(-N(-F)Cl)Br"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::Bond {
                        kind: BondKind::Single,
                        target: Target::Atom(Atom {
                            kind: AtomKind::Aliphatic(Aliphatic::N),
                            links: vec![
                                Link::single_atom(
                                    AtomKind::Aliphatic(Aliphatic::F)
                                ),
                                Link::elided_atom(
                                    AtomKind::Aliphatic(Aliphatic::Cl)
                                )
                            ]
                        })
                    },
                    Link::elided_atom(AtomKind::Aliphatic(Aliphatic::Br))
                ]
            },
            trace: vec![ 0, 3, 6, 8, 11 ]
        }))
    }

    #[test]
    fn branch_split_singleton() {
        assert_eq!(read("C(.O)N"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::Split(Atom::aliphatic(Aliphatic::O)),
                    Link::elided_atom(AtomKind::Aliphatic(Aliphatic::N))
                ]
            },
            trace: vec![ 0, 3, 5 ]
        }))
    }

    #[test]
    fn cut_digit_chain() {
        assert_eq!(read("C1N"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::Bond {
                        kind: BondKind::Elided,
                        target: Target::Join(1)
                    },
                    Link::elided_atom(AtomKind::Aliphatic(Aliphatic::N))
                ]
            },
            trace: vec![ 0, 2 ]
        }))
    }

    #[test]
    fn cut_digit_bond_digit_chain() {
        assert_eq!(read("C1-2N"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::elided_join(1),
                    Link::single_join(2),
                    Link::elided_atom(AtomKind::Aliphatic(Aliphatic::N))
                ]
            },
            trace: vec![ 0, 4 ]
        }))
    }
}