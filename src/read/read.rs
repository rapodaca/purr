use crate::walk::Follower;
use crate::feature::{ AtomKind, BondKind };
use super::{
    Scanner, Trace, Error, missing_character, read_rnum, read_bond,
    read_organic, read_bracket
};

/// Reads a string using a `Follower` and optional `Trace`.
/// 
/// ```
/// use purr::write::Writer;
/// use purr::read::{ read, Error, Trace };
///
/// fn main() -> Result<(), Error> {
///     let mut writer = Writer::new();
///     let mut trace = Trace::new();
///
///     read("CC(=O)N", &mut writer, Some(&mut trace))?;
/// 
///     assert_eq!(writer.write(), "CC(=O)N");
///     assert_eq!(trace.bond(1, 2), Some(3));
///
///     Ok(())
/// }
/// ```
pub fn read<F: Follower>(
    smiles: &str, follower: &mut F, mut trace: Option<&mut Trace>
) -> Result<(), Error> {
    let mut scanner = Scanner::new(smiles);

    if read_smiles(None, &mut scanner, follower, &mut trace)?.is_some() {
        if scanner.is_done() {
            Ok(())
        } else {
            Err(Error::Character(scanner.cursor()))
        }
    } else {
        if scanner.is_done() {
            Err(Error::EndOfLine)
        } else {
            Err(Error::Character(scanner.cursor()))
        }
    }
}

// <smiles> ::= <atom> <body>*
fn read_smiles<F: Follower>(
    input: Option<BondKind>,
    scanner: &mut Scanner,
    follower: &mut F,
    trace: &mut Option<&mut Trace>
) -> Result<Option<usize>, Error> {
    let cursor = scanner.cursor();
    let atom_kind = match read_atom(scanner)? {
        Some(kind) => kind,
        None => return Ok(None)
    };

    match input {
        Some(bond_kind) => {
            if let Some(trace) = trace {
                if bond_kind == BondKind::Elided {
                    trace.extend(cursor, cursor..scanner.cursor())
                } else {
                    trace.extend(cursor - 1, cursor..scanner.cursor())
                }
            }

            follower.extend(bond_kind, atom_kind)
        },
        None => {
            follower.root(atom_kind);

            if let Some(trace) = trace {
                trace.root(cursor..scanner.cursor())
            }
        }
    }

    let mut result = 1;

    loop {
        match read_body(scanner, follower, trace)? {
            Some(length) => result += length,
            None => break Ok(Some(result))
        }
    }
}

// <atom> ::= <organic> | <bracket> | <star>
fn read_atom(
    scanner: &mut Scanner
) -> Result<Option<AtomKind>, Error> {
    if let Some(organic) = read_organic(scanner)? {
        return Ok(Some(organic))
    }

    if let Some(bracket) = read_bracket(scanner)? {
        return Ok(Some(bracket))
    }

    Ok(read_star(scanner))
}

// <body> ::= <branch> | <split> | <union>
fn read_body<F: Follower>(
    scanner: &mut Scanner, follower: &mut F, trace: &mut Option<&mut Trace>
) -> Result<Option<usize>, Error> {
    if read_branch(scanner, follower, trace)? {
        return Ok(Some(0))
    }

    if let Some(length) = read_split(scanner, follower, trace)? {
        return Ok(Some(length))
    }

    read_union(scanner, follower, trace)
}

// <branch> ::= "(" ( <dot> | <bond> )? <smiles> ")"
fn read_branch<F: Follower>(
    scanner: &mut Scanner, follower: &mut F, trace: &mut Option<&mut Trace>
) -> Result<bool, Error> {
    match scanner.peek() {
        Some('(') => {
            scanner.pop();
        },
        _ => return Ok(false)
    }

    let length = match scanner.peek() {
        Some('.') => {
            scanner.pop();

            match read_smiles(None, scanner, follower, trace)? {
                Some(length) => length,
                None => return Err(missing_character(scanner))
            }
        },
        _ => {
            let bond_kind = read_bond(scanner);

            match read_smiles(Some(bond_kind), scanner, follower, trace)? {
                Some(length) => length,
                None => return Err(missing_character(scanner))
            }
        }
    };
    
    match scanner.peek() {
        Some(')') => {
            scanner.pop();
            follower.pop(length);

            if let Some(trace) = trace {
                trace.pop(length)
            }

            Ok(true)
        },
        _ => Err(missing_character(scanner))
    }
}

// <split> ::= <dot> <smiles>
fn read_split<F: Follower>(
    scanner: &mut Scanner, follower: &mut F, trace: &mut Option<&mut Trace>
) -> Result<Option<usize>, Error> {
    match scanner.peek() {
        Some('.') => {
            scanner.pop();
        },
        _ => return Ok(None)
    }

    match read_smiles(None, scanner, follower, trace)? {
        Some(length) => Ok(Some(length)),
        None => Err(missing_character(scanner))
    }
}

// <union> ::= <bond>? ( <smiles> | <rnum> )
fn read_union<F: Follower>(
    scanner: &mut Scanner, follower: &mut F, trace: &mut Option<&mut Trace>
) -> Result<Option<usize>, Error> {
    let bond_cursor = scanner.cursor();
    let bond_kind = read_bond(scanner);

    if let Some(length) = read_smiles(
        Some(bond_kind.clone()), scanner, follower, trace
    )? {
        return Ok(Some(length))
    }

    let cursor = scanner.cursor();

    match read_rnum(scanner)? {
        Some(rnum) => {
            if let Some(trace) = trace {
                trace.join(bond_cursor, cursor..scanner.cursor(), rnum.clone())
            }

            follower.join(bond_kind, rnum);

            Ok(Some(0))
        },
        None => if bond_kind == BondKind::Elided {
            Ok(None)
        } else {
            Err(missing_character(scanner))
        }
    }
}

// <star> = "*"
fn read_star(
    scanner: &mut Scanner
) -> Option<AtomKind> {
    match scanner.peek() {
        Some('*') => {
            scanner.pop();

            Some(AtomKind::Star)
        },
        _ => None
    }
}

#[cfg(test)]
mod read {
    use pretty_assertions::assert_eq;
    use crate::write::Writer;
    use super::*;

    #[test]
    fn blank() {
        let mut writer = Writer::new();

        assert_eq!(
            read("", &mut writer, None),
            Err(Error::EndOfLine)
        )
    }

    #[test]
    fn leading_paren() {
        let mut writer = Writer::new();

        assert_eq!(
            read("(", &mut writer, None),
            Err(Error::Character(0))
        )
    }

    #[test]
    fn invalid_tail() {
        let mut writer = Writer::new();

        assert_eq!(
            read("*?", &mut writer, None),
            Err(Error::Character(1))
        )
    }

    #[test]
    fn trailing_bond() {
        let mut writer = Writer::new();

        assert_eq!(
            read("*-", &mut writer, None),
            Err(Error::EndOfLine)
        )
    }

    #[test]
    fn trailing_dot() {
        let mut writer = Writer::new();

        assert_eq!(
            read("*.", &mut writer, None),
            Err(Error::EndOfLine)
        )
    }

    #[test]
    fn cut_percent_single_digit() {
        let mut writer = Writer::new();

        assert_eq!(
            read("*%1*", &mut writer, None),
            Err(Error::Character(3))
        )
    }

    #[test]
    fn open_paren_eol() {
        let mut writer = Writer::new();

        assert_eq!(
            read("*(", &mut writer, None),
            Err(Error::EndOfLine)
        )
    }

    #[test]
    fn missing_close_paren() {
        let mut writer = Writer::new();

        assert_eq!(
            read("*(*", &mut writer, None),
            Err(Error::EndOfLine)
        )
    }

    #[test]
    fn bond_to_invalid() {
        let mut writer = Writer::new();

        assert_eq!(
            read("*-X", &mut writer, None),
            Err(Error::Character(2))
        )
    }

    #[test]
    fn split_to_invalid() {
        let mut writer = Writer::new();

        assert_eq!(
            read("*.X", &mut writer, None),
            Err(Error::Character(2))
        )
    }

    #[test]
    fn branch_invalid() {
        let mut writer = Writer::new();

        assert_eq!(
            read("*(X)", &mut writer, None),
            Err(Error::Character(2))
        )
    }

    #[test]
    fn branch_rnum() {
        let mut writer = Writer::new();

        assert_eq!(
            read("*(1)*", &mut writer, None),
            Err(Error::Character(2))
        )
    }

    #[test]
    fn branch_bond_rnum() {
        let mut writer = Writer::new();

        assert_eq!(
            read("*(-1)*", &mut writer, None),
            Err(Error::Character(3))
        )
    }

    #[test]
    fn dot_rnum() {
        let mut writer = Writer::new();

        assert_eq!(
            read("*.1", &mut writer, None),
            Err(Error::Character(2))
        )
    }

    #[test]
    fn branch_split_invalid() {
        let mut writer = Writer::new();

        assert_eq!(
            read("*(.X)", &mut writer, None),
            Err(Error::Character(3))
        )
    }

    #[test]
    fn p1() {
        let mut writer = Writer::new();

        read("*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*")
    }

    #[test]
    fn aliphatic_organic() {
        let mut writer = Writer::new();

        read("C", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "C")
    }

    #[test]
    fn aromatic_organic() {
        let mut writer = Writer::new();

        read("c", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "c")
    }

    #[test]
    fn bracket() {
        let mut writer = Writer::new();

        read("[CH4]", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "[CH4]")
    }

    #[test]
    fn elided_rnum() {
        let mut writer = Writer::new();

        read("*1", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*1")
    }

    #[test]
    fn single_rnum() {
        let mut writer = Writer::new();

        read("*-1", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*-1")
    }

    #[test]
    fn p1_p1() {
        let mut writer = Writer::new();

        read("*.*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*.*")
    }

    #[test]
    fn p1_p2_branched_inner() {
        let mut writer = Writer::new();

        read("*(.*)*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*(.*)*")
    }

    #[test]
    fn p2() {
        let mut writer = Writer::new();

        read("*-*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*-*")
    }

    #[test]
    fn p3() {
        let mut writer = Writer::new();

        read("**-*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "**-*")
    }

    #[test]
    fn p3_branched() {
        let mut writer = Writer::new();

        read("*(-*)=*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*(-*)=*")
    }

    #[test]
    fn p4_branched_inside() {
        let mut writer = Writer::new();

        read("*(-**)=*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*(-**)=*")
    }

    #[test]
    fn p4_branched_outside() {
        let mut writer = Writer::new();

        read("*(-*)=**", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*(-*)=**")
    }

    #[test]
    fn nested() {
        let mut writer = Writer::new();

        read("*(*(*-*)*)*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*(*(*-*)*)*")
    }

    #[test]
    fn s4_inside() {
        let mut writer = Writer::new();

        read("*(-*)(=*)(#*)*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*(-*)(=*)(#*)*")
    }

    #[test]
    fn s4_outside() {
        let mut writer = Writer::new();

        read("**(-*)(=*)*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "**(-*)(=*)*")
    }

    #[test]
    fn foo() {
        let mut writer = Writer::new();

        read("C(F)Cl", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "C(F)Cl")
    }
}

#[cfg(test)]
mod trace {
    use pretty_assertions::assert_eq;
    use crate::write::Writer;
    use super::*;

    #[test]
    fn p1() {
        let mut trace = Trace::new();
        let mut writer = Writer::new();

        read("*", &mut writer, Some(&mut trace)).unwrap();

        assert_eq!(trace.atom(0), Some(0..1))
    }

    #[test]
    fn p2() {
        let mut trace = Trace::new();
        let mut writer = Writer::new();

        read("**", &mut writer, Some(&mut trace)).unwrap();

        assert_eq!(trace.atom(0), Some(0..1));
        assert_eq!(trace.atom(1), Some(1..2));
        assert_eq!(trace.bond(0, 1), Some(1));
        assert_eq!(trace.bond(1, 0), Some(1))
    }

    #[test]
    fn p2_single() {
        let mut trace = Trace::new();
        let mut writer = Writer::new();

        read("*-*", &mut writer, Some(&mut trace)).unwrap();

        assert_eq!(trace.atom(0), Some(0..1));
        assert_eq!(trace.atom(1), Some(2..3));
        assert_eq!(trace.bond(0, 1), Some(1));
        assert_eq!(trace.bond(1, 0), Some(1))
    }

    #[test]
    fn p3_branched() {
        let mut trace = Trace::new();
        let mut writer = Writer::new();

        //    01234
        read("*(*)*", &mut writer, Some(&mut trace)).unwrap();

        assert_eq!(trace.atom(0), Some(0..1));
        assert_eq!(trace.atom(1), Some(2..3));
        assert_eq!(trace.atom(2), Some(4..5));
        assert_eq!(trace.bond(0, 1), Some(2));
        assert_eq!(trace.bond(1, 0), Some(2));
        assert_eq!(trace.bond(0, 2), Some(4));
        assert_eq!(trace.bond(2, 0), Some(4))
    }

    #[test]
    fn c3() {
        let mut trace = Trace::new();
        let mut writer = Writer::new();

        //    01234
        read("*1**1", &mut writer, Some(&mut trace)).unwrap();

        assert_eq!(trace.atom(0), Some(0..1));
        assert_eq!(trace.atom(1), Some(2..3));
        assert_eq!(trace.atom(2), Some(3..4));
        assert_eq!(trace.bond(0, 1), Some(2));
        assert_eq!(trace.bond(1, 0), Some(2));
        assert_eq!(trace.bond(1, 2), Some(3));
        assert_eq!(trace.bond(2, 1), Some(3));
        assert_eq!(trace.bond(2, 0), Some(4));
    }
}