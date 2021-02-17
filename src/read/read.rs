use crate::tree::{ Atom, Link, Target };
use crate::parts::{ BondKind };
use super::{
    scanner::Scanner,
    Error,
    Trace,
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
/// use purr::read::{ read, Trace, Error };
/// use purr::parts::{ AtomKind, Aromatic };
/// 
/// fn main() -> Result<(), Error> {
///     let mut trace = Trace::new();
///     let root = read("c1ccccc1", Some(&mut trace))?;
/// 
///     assert_eq!(root.kind, AtomKind::Aromatic(Aromatic::C));
///     assert_eq!(trace, Trace {
///         atoms: vec![ 0..1, 2..3, 3..4, 4..5, 5..6, 6..7 ],
///         bonds: vec![ 1, 2, 3, 4, 5, 6, 7 ],
///         rnums: vec![ 1..2, 7..8 ]
///     });
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
pub fn read(smiles: &str, mut trace: Option<&mut Trace>) -> Result<Atom, Error> {
    let mut scanner = Scanner::new(smiles);
    let root = match read_smiles(None, &mut scanner, &mut trace)? {
        Some(root) => root,
        None => if scanner.is_done() {
            return Err(Error::EndOfLine)
        } else {
            return Err(Error::InvalidCharacter(scanner.cursor()))
        }
    };

    if scanner.is_done() {
        Ok(root)
    } else {
        Err(Error::InvalidCharacter(scanner.cursor()))
    }
}

// <smiles> ::= <atom> <body>*
fn read_smiles(
    input_cursor: Option<usize>, scanner: &mut Scanner, trace: &mut Option<&mut Trace>
) -> Result<Option<Atom>, Error> {
    let mut root = match read_atom(scanner, trace)? {
        Some(root) => root,
        None => return Ok(None)
    };

    if let Some(trace) = trace {
        if let Some(cursor) = input_cursor {
            trace.bonds.push(cursor)
        }
    }

    while let Some(body) = read_body(scanner, trace)? {
        root.links.push(body);
    }

    Ok(Some(root))
}

// <atom> ::= <organic> | <bracket> | <star>
fn read_atom(
    scanner: &mut Scanner, trace: &mut Option<&mut Trace>
) -> Result<Option<Atom>, Error> {
    let start = scanner.cursor();
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

    if let Some(trace) = trace {
        trace.atoms.push(start..scanner.cursor())
    }

    Ok(Some(result))
}

// <body> ::= <branch> | <split> | <union>
fn read_body(
    scanner: &mut Scanner, trace: &mut Option<&mut Trace>
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
    trace: &mut Option<&mut Trace>
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

            match read_smiles(None, scanner, trace)? {
                Some(target) => Link::Split(target),
                None => return Err(missing_character(scanner))
            }
        },
        _ => {
            let cursor = scanner.cursor();
            let kind = read_bond(scanner);

            match read_smiles(Some(cursor), scanner, trace)? {
                Some(target) => {
                    Link::Bond {
                        kind,
                        target: Target::Atom(target)
                    }
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
    trace: &mut Option<&mut Trace>
) -> Result<Option<Link>, Error> {
    match scanner.peek() {
        Some('.') => {
            scanner.pop();
        },
        _ => return Ok(None)
    }

    match read_smiles(None, scanner, trace)? {
        Some(smiles) => Ok(Some(Link::Split(smiles))),
        _ => Err(missing_character(scanner))
    }
}

// <union> ::= <bond>? ( <smiles> | <rnum> )
fn read_union(
    scanner: &mut Scanner,
    trace: &mut Option<&mut Trace>
) -> Result<Option<Link>, Error> {
    let start_bond = scanner.cursor();
    let bond_kind = read_bond(scanner);

    let target = if let Some(smiles) = read_smiles(Some(start_bond), scanner, trace)? {
        Target::Atom(smiles)
    } else {
        let start_rnum = scanner.cursor();

        if let Some(rnum) = read_rnum(scanner)? {
            if let Some(trace) = trace {
                trace.bonds.push(start_bond);
                trace.rnums.push(start_rnum..scanner.cursor())
            }
            
            Target::Join(rnum)
        } else if bond_kind == BondKind::Elided {
            return Ok(None)
        } else {
            return Err(missing_character(scanner))
        }
    };

    Ok(Some(Link::Bond {
        kind: bond_kind,
        target: target
    }))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::parts::{
        Element, AtomKind, Aliphatic, BracketSymbol, Aromatic, VirtualHydrogen
    };
    use crate::tree::{ Rnum };
    use super::*;

    #[test]
    fn blank() {
        assert_eq!(read("", None), Err(Error::EndOfLine))
    }

    #[test]
    fn leading_paren() {
        assert_eq!(read("(", None), Err(Error::InvalidCharacter(0)))
    }

    #[test]
    fn invalid_tail() {
        assert_eq!(read("C?", None), Err(Error::InvalidCharacter(1)))
    }

    #[test]
    fn trailing_bond() {
        assert_eq!(read("*-", None), Err(Error::EndOfLine))
    }

    #[test]
    fn trailing_dot() {
        assert_eq!(read("*.", None), Err(Error::EndOfLine))
    }

    #[test]
    fn cut_percent_single_digit() {
        assert_eq!(read("C%1N", None), Err(Error::InvalidCharacter(3)))
    }

    #[test]
    fn open_paren_eol() {
        assert_eq!(read("C(", None), Err(Error::EndOfLine))
    }

    #[test]
    fn missing_close_paren() {
        assert_eq!(read("C(C", None), Err(Error::EndOfLine))
    }

    #[test]
    fn bond_to_invalid() {
        assert_eq!(read("C-X", None), Err(Error::InvalidCharacter(2)))
    }

    #[test]
    fn branch_invalid() {
        assert_eq!(read("C(?)C", None), Err(Error::InvalidCharacter(2)))
    }

    #[test]
    fn branch_rnum() {
        assert_eq!(read("C(1)C", None), Err(Error::InvalidCharacter(2)))
    }

    #[test]
    fn branch_bond_rnum() {
        assert_eq!(read("C(-1)C", None), Err(Error::InvalidCharacter(3)))
    }

    #[test]
    fn dot_rnum() {
        assert_eq!(read("C.1", None), Err(Error::InvalidCharacter(2)));
    }

    #[test]
    fn star() {
        let mut trace = Trace::new();

        assert_eq!(read("*", Some(&mut trace)), Ok(Atom::star()));
        assert_eq!(trace, Trace {
            atoms: vec![ 0..1 ],
            bonds: vec![ ],
            rnums: vec![ ]
        })
    }

    #[test]
    fn methane_bracket() {
        let mut trace = Trace::new();

        assert_eq!(read("[CH4]", Some(&mut trace)), Ok(Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Element(Element::C),
                configuration: None,
                hcount: Some(VirtualHydrogen::H4),
                charge: None,
                map: None
            },
            links: vec![ ]
        }));
        assert_eq!(trace, Trace {
            atoms: vec![ 0..5 ],
            bonds: vec![ ],
            rnums: vec![ ]
        })
    }

    #[test]
    fn aromatic() {
        let mut trace = Trace::new();

        assert_eq!(read("c", Some(&mut trace)), Ok(Atom {
            kind: AtomKind::Aromatic(Aromatic::C),
            links: vec![ ]
        }));
        assert_eq!(trace, Trace {
            atoms: vec![ 0..1 ],
            bonds: vec![ ],
            rnums: vec![ ]
        })
    }

    #[test]
    fn atom_rnum_implicit() {
        let mut trace = Trace::new();

        assert_eq!(read("C1", Some(&mut trace)), Ok(Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            links: vec![ Link::elided_join(Rnum::R1) ]
        }));
        assert_eq!(trace, Trace {
            atoms: vec![ 0..1 ],
            bonds: vec![ 1 ],
            rnums: vec![ 1..2 ]
        })
    }

    #[test]
    fn implicit_bond() {
        let mut trace = Trace::new();

        assert_eq!(read("CO", Some(&mut trace)), Ok(Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            links: vec![
                Link::elided_atom(AtomKind::Aliphatic(Aliphatic::O))
            ]
        }));
        assert_eq!(trace, Trace {
            atoms: vec![ 0..1, 1..2 ],
            bonds: vec![ 1 ],
            rnums: vec![ ]
        })
    }

    #[test]
    fn single_bond() {
        let mut trace = Trace::new();

        assert_eq!(read("C-O", Some(&mut trace)), Ok(Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            links: vec![
                Link::single_atom(AtomKind::Aliphatic(Aliphatic::O))
            ]
        }));

        assert_eq!(trace, Trace {
            atoms: vec![ 0..1, 2..3 ],
            bonds: vec![ 1 ],
            rnums: vec![ ]
        })
    }

    #[test]
    fn double_bond() {
        let mut trace = Trace::new();

        assert_eq!(read("C=O", Some(&mut trace)), Ok(Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            links: vec![
                Link::double_atom(AtomKind::Aliphatic(Aliphatic::O))
            ]
        }));
        assert_eq!(trace, Trace {
            atoms: vec![ 0..1, 2..3 ],
            bonds: vec![  1 ],
            rnums: vec![ ]
        })
    }

    #[test]
    fn ethane_bracket() {
        let mut trace = Trace::new();

        assert_eq!(read("C[CH4]", Some(&mut trace)), Ok(Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            links: vec![ 
                Link::Bond {
                    kind: BondKind::Elided,
                    target: Target::Atom(Atom {
                        kind: AtomKind::Bracket {
                            isotope: None,
                            symbol: BracketSymbol::Element(Element::C),
                            configuration: None,
                            hcount: Some(VirtualHydrogen::H4),
                            charge: None,
                            map: None
                        },
                        links: vec![ ]
                    })
                }
            ]
        }));
        assert_eq!(trace, Trace {
            atoms: vec![ 0..1, 1..6 ],
            bonds: vec![ 1 ],
            rnums: vec![ ]
        })
    }

    #[test]
    fn implicit_aromatic() {
        let mut trace = Trace::new();

        assert_eq!(read("cn", Some(&mut trace)), Ok(Atom {
            kind: AtomKind::Aromatic(Aromatic::C),
            links: vec![
                Link::elided_atom(AtomKind::Aromatic(Aromatic::N))
            ]
        }));
        assert_eq!(trace, Trace {
            atoms: vec![0..1, 1..2 ],
            bonds: vec![ 1 ],
            rnums: vec![ ]
        })
    }

    #[test]
    fn split() {
        let mut trace = Trace::new();

        assert_eq!(read("C.O", Some(&mut trace)), Ok(Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            links: vec![
                Link::Split(Atom::aliphatic(Aliphatic::O))
            ]
        }));
        assert_eq!(trace, Trace {
            atoms: vec![ 0..1, 2..3 ],
            bonds: vec![ ],
            rnums: vec![ ]
        })
    }

    #[test]
    fn branch_singleton_elided() {
        let mut trace = Trace::new();

        assert_eq!(read("C(O)N", Some(&mut trace)), Ok(Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            links: vec![
                Link::elided_atom(AtomKind::Aliphatic(Aliphatic::O)),
                Link::elided_atom(AtomKind::Aliphatic(Aliphatic::N))
            ]
        }));
        assert_eq!(trace, Trace {
            atoms: vec![ 0..1, 2..3, 4..5 ],
            bonds: vec![ 2, 4 ],
            rnums: vec![ ]
        })
    }

    #[test]
    fn branch_singleton_double() {
        let mut trace = Trace::new();

        assert_eq!(read("C(=O)N", Some(&mut trace)), Ok(Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            links: vec![
                Link::Bond {
                    kind: BondKind::Double,
                    target: Target::Atom(Atom::aliphatic(Aliphatic::O))
                },
                Link::elided_atom(AtomKind::Aliphatic(Aliphatic::N))
            ]
        }));
        assert_eq!(trace, Trace {
            atoms: vec![ 0..1, 3..4, 5..6 ],
            bonds: vec![ 2, 5 ],
            rnums: vec![ ]
        })
    }

    #[test]
    fn branch_chain_single() {
        let mut trace = Trace::new();

        assert_eq!(read("C(-OC)N", Some(&mut trace)), Ok(Atom {
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
        }));
        assert_eq!(trace, Trace {
            atoms: vec![ 0..1, 3..4, 4..5, 6..7 ],
            bonds: vec![ 2, 4, 6],
            rnums: vec![ ]
        })
    }

    #[test]
    fn branch_branch_single() {
        let mut trace = Trace::new();
        //               0123456789012
        assert_eq!(read("C(-N(-F)Cl)Br", Some(&mut trace)), Ok(Atom {
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
        }));
        assert_eq!(trace, Trace {
            atoms: vec![ 0..1, 3..4, 6..7, 8..10, 11..13 ],
            bonds: vec![ 2, 5, 8, 11 ],
            rnums: vec![ ]
        })
    }

    #[test]
    fn branch_split_singleton() {
        let mut trace = Trace::new();

        assert_eq!(read("C(.O)N", Some(&mut trace)), Ok(Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            links: vec![
                Link::Split(Atom::aliphatic(Aliphatic::O)),
                Link::elided_atom(AtomKind::Aliphatic(Aliphatic::N))
            ]
        }));
        assert_eq!(trace, Trace {
            atoms: vec![ 0..1, 3..4, 5..6 ],
            bonds: vec![ 5 ],
            rnums: vec![ ]
        })
    }

    #[test]
    fn cut_digit_chain() {
        let mut trace = Trace::new();

        assert_eq!(read("C1N", Some(&mut trace)), Ok(Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            links: vec![
                Link::Bond {
                    kind: BondKind::Elided,
                    target: Target::Join(Rnum::R1)
                },
                Link::elided_atom(AtomKind::Aliphatic(Aliphatic::N))
            ]
        }));
        assert_eq!(trace, Trace {
            atoms: vec![ 0..1, 2..3 ],
            bonds: vec![ 1, 2 ],
            rnums: vec![ 1..2 ]
        })
    }

    #[test]
    fn cut_digit_bond_digit_chain() {
        let mut trace = Trace::new();

        assert_eq!(read("C1-2N", Some(&mut trace)), Ok(Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            links: vec![
                Link::elided_join(Rnum::R1),
                Link::single_join(Rnum::R2),
                Link::elided_atom(AtomKind::Aliphatic(Aliphatic::N))
            ]
        }));
        assert_eq!(trace, Trace {
            atoms: vec![ 0..1, 4..5 ],
            bonds: vec![ 1, 2, 4 ],
            rnums: vec![ 1..2, 3..4 ]
        })
    }
}