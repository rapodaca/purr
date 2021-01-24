use crate::parts::{ BondKind };
use crate::tree::{ Target, Atom, Link  };
use super::{
    Reading, Scanner, Error, missing_character,
    read_bare_atom, read_bracket_atom, read_bond,
    read_rnum
};

pub fn read(smiles: &str) -> Result<Reading, Error> {
    let mut scanner = Scanner::new(smiles);
    let mut trace = Vec::new();

    match read_run(&mut scanner, &mut trace)? {
        Some(root) => if scanner.is_done() {
            Ok(Reading {
                root: root,
                trace: trace
            })
        } else {
            Err(Error::InvalidCharacter(scanner.cursor()))
        },
        None => Err(missing_character(&mut scanner))
    }
}

// <run>    ::= <atom> <tail>*
fn read_run(
    scanner: &mut Scanner, trace: &mut Vec<usize>
) -> Result<Option<Atom>, Error> {
    let mut atom = match read_atom(scanner, trace)? {
        Some(atom) => atom,
        None => return Ok(None)
    };

    loop {
        match read_tail(scanner, trace)? {
            Some(tail) => {
                atom.links.push(tail);
            },
            None => break
        }
    }

    Ok(Some(atom))
}

// <tail>   ::= <branch> | <block>
fn read_tail(
    scanner: &mut Scanner, trace: &mut Vec<usize>
) -> Result<Option<Link>, Error> {
    if let Some(branch) = read_branch(scanner, trace)? {
        Ok(Some(branch))
    } else {
        read_block(scanner, trace)
    }
}

// <block>  ::= <split> | <chain>
fn read_block(
    scanner: &mut Scanner, trace: &mut Vec<usize>
) -> Result<Option<Link>, Error> {
    if let Some(split) = read_split(scanner, trace)? {
        Ok(Some(split))
    } else {
        read_chain(scanner, trace)
    }
}

// <split>  ::= <dot> <run>
fn read_split(
    scanner: &mut Scanner, trace: &mut Vec<usize>
) -> Result<Option<Link>, Error> {
    match scanner.peek() {
        Some('.') => {
            scanner.pop();
        },
        _ => return Ok(None)
    }

    if let Some(run) = read_run(scanner, trace)? {
        Ok(Some(Link::Split(run)))
    } else {
        Err(missing_character(scanner))
    }
}

// <chain>  ::= <bond>? ( <run> | <rnum> )
fn read_chain(
    scanner: &mut Scanner, trace: &mut Vec<usize>
) -> Result<Option<Link>, Error> {
    let bond = read_bond(scanner);
    let mut trace2 = Vec::new();

    let target = if let Some(run) = read_run(scanner, &mut trace2)? {
        Some(Target::Atom(run))
    } else if let Some(cut) = read_rnum(scanner)? {
        Some(Target::Join(cut))
    } else {
        None
    };

    if let Some(target) = target {
        trace.append(&mut trace2);

        Ok(Some(Link::Bond { kind: bond, target }))
    } else if let BondKind::Elided = bond {
        Ok(None)
    } else {
        Err(missing_character(scanner))
    }
}

// <branch> ::= "(" <block> ")"
fn read_branch(
    scanner: &mut Scanner, trace: &mut Vec<usize>
) -> Result<Option<Link>, Error> {
    match scanner.peek() {
        Some('(') => {
            scanner.pop();
        },
        _ => return Ok(None)
    }

    let result = match read_block(scanner, trace)? {
        Some(link) => link,
        None => return Err(missing_character(scanner))
    };

    match scanner.peek() {
        Some(')') => {
            scanner.pop();

            Ok(Some(result))
        },
        _ => Err(missing_character(scanner))
    }
}

fn read_atom(
    scanner: &mut Scanner, trace: &mut Vec<usize>
) -> Result<Option<Atom>, Error> {
    let cursor = scanner.cursor();
    let atom = match read_bare_atom(scanner)? {
        Some(atom) => Some(atom),
        None => read_bracket_atom(scanner)?
    };

    if atom.is_some() {
        trace.push(cursor)
    }

    Ok(atom)
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
        assert_eq!(read("C(?)C"), Err(Error::InvalidCharacter(2)));
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
    fn atom_rnum() {
        assert_eq!(read("C1"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![ Link::elided_join(1) ]
            },
            trace: vec![ 0 ]
        }))
    }

    #[test]
    fn atom_rnum_branch() {
        assert_eq!(read("C(1)"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::elided_join(1)
                ]
            },
            trace: vec![ 0 ]
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
    fn implicit_single() {
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
    fn branch_singleton() {
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
    fn branch_singleton_bond() {
        assert_eq!(read("C(-O)N"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::single_atom(AtomKind::Aliphatic(Aliphatic::O)),
                    Link::elided_atom(AtomKind::Aliphatic(Aliphatic::N))
                ]
            },
            trace: vec![ 0, 3, 5 ]
        }))
    }

    #[test]
    fn branch_chain() {
        assert_eq!(read("C(OC)N"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::Bond {
                        kind: BondKind::Elided,
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
            trace: vec![ 0, 2, 3, 5 ]
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

    #[test]
    fn cut_digit_parens_digit_chain() {
        assert_eq!(read("C(1)N"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::elided_join(1),
                    Link::elided_atom(AtomKind::Aliphatic(Aliphatic::N))
                ]
            },
            trace: vec![ 0, 4 ]
        }))
    }

    #[test]
    fn split() {
        assert_eq!(read("C.O"), Ok(Reading {
            root: Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                links: vec![
                    Link::Split(Atom::aliphatic(Aliphatic::O))
                ]
            },
            trace: vec![ 0, 2 ]
        }))
    }
}