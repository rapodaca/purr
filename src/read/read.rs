use crate::util::Scanner;
use crate::mol::{ Builder, Style, Atom, Nub, Error as MolError };
use super::error::Error;
use super::bare_atom::bare_atom;
use super::bracket_atom::bracket_atom;

/// Reads a string representation of a SMILES. Supports the feature set found
/// in *[SMILES Formal Grammar](https://depth-first.com/articles/2020/04/20/smiles-formal-grammar/)*.
/// This is a subset of
/// features supported by [OpenSMILES](http://opensmiles.org). The missing
/// features are:
///
/// - no wildcard/unknown atom (*)
/// - only tetrahedral atom pariities (@, @@)
/// 
/// ```rust
/// use purr::read::{ read, Error };
/// use purr::valence::implicit_hydrogens;
/// use purr::mol::{ Atom, Bond, Nub, Element };
///
/// fn main() -> Result<(), Error> {
///     let atoms = read(&"OC[CH3]")?;
///     
///     assert_eq!(atoms, vec![
///         Atom {
///             nub: Nub { element: Element::O, ..Default::default() },
///             bonds: vec![
///                 Bond { tid: 1, style: None }
///             ]
///         },
///         Atom {
///             nub: Default::default(),
///             bonds: vec![
///                 Bond { tid: 0, style: None },
///                 Bond { tid: 2, style: None }
///             ]
///         },
///         Atom {
///             nub: Nub {
///                 hcount: Some(3), charge: Some(0), ..Default::default()
///             },
///             bonds: vec![
///                 Bond { tid: 1, style: None }
///             ]
///         }
///     ]);
///
///     assert_eq!(implicit_hydrogens(&atoms[0]), Ok(Some(1)));
///     assert_eq!(implicit_hydrogens(&atoms[1]), Ok(Some(2)));
///     assert_eq!(implicit_hydrogens(&atoms[2]), Ok(None));
///
///     Ok(())
/// }
/// ```
pub fn read(text: &str) -> Result<Vec<Atom>, Error> {
    let mut scanner = Scanner::new(text);

    if scanner.done() {
        return Err(Error::EndOfLine);
    }
    
    if let Some(atom) = either_atom(&mut scanner)? {
        let mut state = State {
            scanner: scanner,
            builder: Builder::new(atom),
            dot: false
        };

        loop {
            if !chain(&mut state)? && !branch(&mut state)? {
                if !state.scanner.done() {
                    break Err(Error::InvalidCharacter(state.scanner.cursor()))
                }

                break match state.builder.to_atoms() {
                    Ok(atoms) => Ok(atoms),
                    Err(MolError::OpenCycles(_)) => Err(Error::EndOfLine),
                    Err(error) => Err(Error::MolError(state.scanner.cursor(), error))
                }
            }
        }
    } else {
        Err(Error::InvalidCharacter(0))
    }
}

fn either_atom(scanner: &mut Scanner) -> Result<Option<Nub>, Error> {
    match bare_atom(scanner)? {
        Some(atom) => Ok(Some(atom)),
        None => bracket_atom(scanner)
    }
}

// <line> ::= <atom> ( <chain> | <branch> )*
fn line(state: &mut State) -> Result<bool, Error> {
    if !atom(state)? {
        return Ok(false);
    }

    loop {
        if !chain(state)? && !branch(state)? {
            break Ok(true)
        }
    }
}

// <atom> ::= <organic_symbol> | <bracket_atom>
fn atom(state: &mut State) -> Result<bool, Error> {
    let dot = state.dot;
    state.dot = false;

    match either_atom(&mut state.scanner)? {
        Some(atom) => {
            if dot {
                state.builder.root(atom);
            } else {
                state.builder.extend(atom);
            }

            Ok(true)
        },
        None => Ok(false)
    }
}

// <chain> ::= ( <dot> <atom> | <bond>? ( <atom> | <rnum>) )+
fn chain(state: &mut State) -> Result<bool, Error> {
    let mut result = false;

    loop {
        if dot(state)? {
            if !atom(state)? {
                break Err(Error::InvalidCharacter(state.scanner.cursor()))
            }
        } else if bond(state)? {
            if !atom(state)? && !rnum(state)? {
                break Err(Error::InvalidCharacter(state.scanner.cursor()))
            }
        } else {
            if !atom(state)? && !rnum(state)? {
                break Ok(result)
            }
        }

        result = true;
    }
}

// <dot> ::= "."
fn dot(state: &mut State) -> Result<bool, Error> {
    match state.scanner.peek() {
        Some('.') => {
            state.dot = true;

            state.scanner.pop();

            Ok(true)
        },
        _ => Ok(false)
    }
}

// <rnum> ::= <digit> | "%" <digit> <digit>
fn rnum(state: &mut State) -> Result<bool, Error> {
    let cursor = state.scanner.cursor();

    match digits(state)? {
        Some(rnum) => {
            match state.builder.cut(rnum) {
                Ok(()) => Ok(true),
                Err(error) => Err(Error::MolError(cursor, error))
            }
        },
        None => Ok(false)
    }
}

fn digits(state: &mut State) -> Result<Option<u8>, Error> {
    let mut digits = String::new();

    match state.scanner.peek() {
        Some('0'..='9') => {
            digits.push(*state.scanner.pop().unwrap());
        },
        Some('%') => {
            for _ in 0..=1 {
                match state.scanner.peek() {
                    Some('0'..='9') => {
                        digits.push(*state.scanner.pop().unwrap());
                    }
                    Some(_) => return Err(
                        Error::InvalidCharacter(state.scanner.cursor())
                    ),
                    None => return Err(Error::EndOfLine)
                }
            }
        },
        _ => return Ok(None)
    }

    Ok(Some(digits.parse::<u8>().unwrap()))
}

// <branch> ::= "(" ( <bond>? <line> )+ ")"
fn branch(state: &mut State) -> Result<bool, Error> {
    if let Some('(') = state.scanner.peek() {
        state.scanner.pop();
    } else {
        return Ok(false);
    }

    state.builder.open();

    while !state.scanner.done() {
        bond(state)?;

        if !line(state)? {
            return Err(Error::InvalidCharacter(state.scanner.cursor()));
        }

        if let Some(')') = state.scanner.peek() {
            break;
        }
    }

    state.builder.close();

    match state.scanner.pop() {
        Some(')') => Ok(true),
        Some(_) => Err(Error::InvalidCharacter(state.scanner.cursor())),
        None => Err(Error::EndOfLine)
    }
}

// <bond> ::= "-" | "=" | "#" | "$" | "/" | "\\"
fn bond(state: &mut State) -> Result<bool, Error> {
    let style = match state.scanner.peek() {
        Some('-') => Some(Style::Single),
        Some('=') => Some(Style::Double),
        Some('#') => Some(Style::Triple),
        Some('$') => Some(Style::Quadruple),
        Some(':') => Some(Style::Aromatic),
        Some('/') => Some(Style::Up),
        Some('\\') => Some(Style::Down),
        _ => None
    };

    match style {
        Some(style) => {
            state.scanner.pop();
            state.builder.bond(style);

            Ok(true)
        },
        None => Ok(false)
    }
}

struct State {
    scanner: Scanner,
    builder: Builder,
    dot: bool
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mol::Element;
    use crate::mol::Bond;
    use crate::mol::Parity;

    #[test]
    fn blank() {
        assert_eq!(read(&""), Err(Error::EndOfLine));
    }

    #[test]
    fn unrecognized_ending() {
        let atoms = read(&"CX");

        assert_eq!(atoms, Err(Error::InvalidCharacter(1)));
    }
    
    #[test]
    fn open_paren() {
        let mol = read(&"C(C");

        assert_eq!(mol, Err(Error::EndOfLine));
    }

    #[test]
    fn open_paren_unrecognized_ending() {
        let mol = read(&"C(CX");

        assert_eq!(mol, Err(Error::InvalidCharacter(3)));
    }

    #[test]
    fn open_cycle() {
        let mol = read(&"C1CC");

        assert_eq!(mol, Err(Error::EndOfLine));
    }

    #[test]
    fn mismatched_style() {
        let mol = read(&"C-1CC=1");

        assert_eq!(mol, Err(Error::MolError(6, MolError::MismatchedStyle(2, 0))));
    }

    #[test]
    fn methane() {
        let mol = read(&"C").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Nub { element: Element::C, ..Default::default() },
                bonds: vec![ ]
            }
        ]);
    }

    #[test]
    fn ammonia() {
        let mol = read(&"N").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Nub { element: Element::N, ..Default::default() },
                bonds: vec![ ]
            }
        ]);
    }

    #[test]
    fn kitchen_sink_head() {
        let mol = read(&"[15n@H+:123]").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Nub {
                    element: Element::N, isotope: Some(15), aromatic: true,
                    parity: Some(Parity::Counterclockwise),
                    hcount: Some(1), charge: Some(1), map: 123
                },
                bonds: vec![ ]
            }
        ]);
    }

    #[test]
    fn kitchen_sink_body() {
        let mol = read(&"C.[15n@H+:123]").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(), bonds: vec![ ]
            },
            Atom {
                nub: Nub {
                    element: Element::N, isotope: Some(15), aromatic: true,
                    parity: Some(Parity::Counterclockwise),
                    hcount: Some(1), charge: Some(1), map: 123

                },
                bonds: vec![ ]
            }
        ]);
    }

    #[test]
    fn ethane() {
        let mol = read(&"CC").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 1, style: None } ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 0, style: None } ]
            }
        ]);
    }

    #[test]
    fn ethane_with_explicit_bond() {
        let mol = read(&"C-C").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 1, style: Some(Style::Single) } ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 0, style: Some(Style::Single) } ]
            }
        ]);
    }

    #[test]
    fn ethene() {
        let mol = read(&"C=C").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 1, style: Some(Style::Double) } ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 0, style: Some(Style::Double) } ]
            }
        ]);
    }

    #[test]
    fn aromatic_ethene() {
        let mol = read(&"C:C").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 1, style: Some(Style::Aromatic) } ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 0, style: Some(Style::Aromatic) } ]
            }
        ]);
    }

    #[test]
    fn up_ethane() {
        let mol = read(&"C/C").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 1, style: Some(Style::Up) } ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 0, style: Some(Style::Down) } ]
            }
        ]);
    }

    #[test]
    fn down_ethane() {
        let mol = read(&"C\\C").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 1, style: Some(Style::Down) } ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 0, style: Some(Style::Up) } ]
            }
        ]);
    }

    #[test]
    fn ethyne() {
        let mol = read(&"C#C").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 1, style: Some(Style::Triple) } ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 0, style: Some(Style::Triple) } ]
            }
        ]);
    }

    #[test]
    fn c2() {
        let mol = read(&"C$C").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 1, style: Some(Style::Quadruple) } ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 0, style: Some(Style::Quadruple) } ]
            }
        ]);
    }

    #[test]
    fn methane_hydrate() {
        let mol = read(&"C.O").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![ ]
            },
            Atom {
                nub: Nub { element: Element::O, ..Default::default() },
                bonds: vec![ ]
            }
        ]);
    }

    #[test]
    fn ethane_hydrate() {
        let mol = read(&"O.CC").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Nub { element: Element::O, ..Default::default() },
                bonds: vec![ ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 2, style: None } ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 1, style: None } ]
            }
        ]);
    }

    #[test]
    fn propane() {
        let mol = read(&"CCC").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 1, style: None } ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 0, style: None },
                    Bond { tid: 2, style: None }
                ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 1, style: None } ]
            }
        ]);
    }

    #[test]
    fn branched_propane() {
        let mol = read(&"C(C)C").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 2, style: None }
                ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 0, style: None } ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 0, style: None } ]
            }
        ]);
    }

    #[test]
    fn propene() {
        let mol = read(&"C=CC").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 1, style: Some(Style::Double) } ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 0, style: Some(Style::Double) },
                    Bond { tid: 2, style: None }
                ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![ Bond { tid: 1, style: None } ]
            }
        ]);
    }

    #[test]
    fn bromochloroflurormethane() {
        let mol = read(&"C(F)(Cl)Br").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 2, style: None },
                    Bond { tid: 3, style: None }
                ]
            },
            Atom {
                nub: Nub { element: Element::F, ..Default::default() },
                bonds: vec![
                    Bond { tid: 0, style: None }
                ]
            },
            Atom {
                nub: Nub { element: Element::Cl, ..Default::default() },
                bonds: vec![
                    Bond { tid: 0, style: None }
                ]
            },
            Atom {
                nub: Nub { element: Element::Br, ..Default::default() },
                bonds: vec![
                    Bond { tid: 0, style: None }
                ]
            },
        ]);
    }

    #[test]
    fn nested_parens() {
        //               0 1 2 3 4
        let mol = read(&"C(C(C)C)C").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 4, style: None },
                ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 0, style: None },
                    Bond { tid: 2, style: None },
                    Bond { tid: 3, style: None },
                ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 1, style: None }
                ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 1, style: None }
                ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 0, style: None }
                ]
            },
        ]);
    }

    #[test]
    fn monocycle() {
        //               0 12
        let mol = read(&"C1CC1").unwrap();

        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 2, style: None },
                    Bond { tid: 1, style: None }
                ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 0, style: None },
                    Bond { tid: 2, style: None }
                ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 0, style: None }
                ]
            }
        ]);
    }

    #[test]
    fn bicycle() {
        //               0  12 3
        let mol = read(&"C12CC1C2").unwrap();
        
        assert_eq!(mol, vec![
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 2, style: None },
                    Bond { tid: 3, style: None },
                    Bond { tid: 1, style: None }
                ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 0, style: None },
                    Bond { tid: 2, style: None }
                ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 0, style: None },
                    Bond { tid: 3, style: None },
                ]
            },
            Atom {
                nub: Default::default(),
                bonds: vec![
                    Bond { tid: 2, style: None },
                    Bond { tid: 0, style: None }
                ]
            },
        ]);
    }
}