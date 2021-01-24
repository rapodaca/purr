use crate::parts::{ Aliphatic, Aromatic };
use crate::tree::{ Atom };
use super::{ Scanner, Error };

pub fn read_bare_atom(scanner: &mut Scanner) -> Result<Option<Atom>, Error> {
     match scanner.peek() {
        Some('*') => {
            scanner.pop();

            Ok(Some(Atom::star()))
        },
        Some('b') => aromatic(Aromatic::B, scanner),
        Some('c') => aromatic(Aromatic::C, scanner),
        Some('n') => aromatic(Aromatic::N, scanner),
        Some('o') => aromatic(Aromatic::O, scanner),
        Some('p') => aromatic(Aromatic::P, scanner),
        Some('s') => aromatic(Aromatic::S, scanner),
        Some('A') => {
            scanner.pop();

            match scanner.peek() {
                Some('t') => aliphatic(Aliphatic::At, scanner),
                _ => Err(Error::InvalidCharacter(scanner.cursor()))
            }
        },
        Some('B') => {
            scanner.pop();

            match scanner.peek() {
                Some('r') => aliphatic(Aliphatic::Br, scanner),
                _ => Ok(Some(Atom::aliphatic(Aliphatic::B)))
            }
        },
        Some('C') => {
            scanner.pop();

            match scanner.peek() {
                Some('l') => aliphatic(Aliphatic::Cl, scanner),
                _ => Ok(Some(Atom::aliphatic(Aliphatic::C)))
            }
        },
        Some('N') => aliphatic(Aliphatic::N, scanner),
        Some('O') => aliphatic(Aliphatic::O, scanner),
        Some('P') => aliphatic(Aliphatic::P, scanner),
        Some('S') => aliphatic(Aliphatic::S, scanner),
        Some('F') => aliphatic(Aliphatic::F, scanner),
        Some('I') => aliphatic(Aliphatic::I, scanner),
        Some('T') => {
            scanner.pop();

            match scanner.peek() {
                Some('s') => aliphatic(Aliphatic::Ts, scanner),
                _ => Err(Error::InvalidCharacter(scanner.cursor()))
            }
        }
        _ => Ok(None)
    }
}

fn aromatic(
    aromatic: Aromatic, scanner: &mut Scanner
) -> Result<Option<Atom>, Error> {
    scanner.pop();

    Ok(Some(Atom::aromatic(aromatic)))
}

fn aliphatic(
    aliphatic: Aliphatic, scanner: &mut Scanner
) -> Result<Option<Atom>, Error> {
    scanner.pop();

    Ok(Some(Atom::aliphatic(aliphatic)))
}