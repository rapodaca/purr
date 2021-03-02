use crate::feature::{ Aliphatic, Aromatic, AtomKind };
use super::{
    scanner::Scanner,
    Error,
    missing_character::missing_character
};

pub fn read_organic(scanner: &mut Scanner) -> Result<Option<AtomKind>, Error> {
     match scanner.peek() {
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
                _ => Err(missing_character(scanner))
            }
        },
        Some('B') => {
            scanner.pop();

            match scanner.peek() {
                Some('r') => aliphatic(Aliphatic::Br, scanner),
                _ => Ok(Some(AtomKind::Aliphatic(Aliphatic::B)))
            }
        },
        Some('C') => {
            scanner.pop();

            match scanner.peek() {
                Some('l') => aliphatic(Aliphatic::Cl, scanner),
                _ => Ok(Some(AtomKind::Aliphatic(Aliphatic::C)))
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
                _ => Err(missing_character(scanner))
            }
        }
        _ => Ok(None)
    }
}

fn aromatic(
    aromatic: Aromatic, scanner: &mut Scanner
) -> Result<Option<AtomKind>, Error> {
    scanner.pop();

    Ok(Some(AtomKind::Aromatic(aromatic)))
}

fn aliphatic(
    aliphatic: Aliphatic, scanner: &mut Scanner
) -> Result<Option<AtomKind>, Error> {
    scanner.pop();

    Ok(Some(AtomKind::Aliphatic(aliphatic)))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn a_x() {
        let mut scanner = Scanner::new("Ax");
        let atom = read_organic(&mut scanner);

        assert_eq!(atom, Err(Error::Character(1)))
    }

    #[test]
    fn t_x() {
        let mut scanner = Scanner::new("Tx");
        let atom = read_organic(&mut scanner);

        assert_eq!(atom, Err(Error::Character(1)))
    }

    #[test]
    fn b_x() {
        let mut scanner = Scanner::new("Bx");
        let atom = read_organic(&mut scanner);

        assert_eq!(atom, Ok(Some(AtomKind::Aliphatic(Aliphatic::B))))
    }

    #[test]
    fn c_x() {
        let mut scanner = Scanner::new("Cx");
        let atom = read_organic(&mut scanner);

        assert_eq!(atom, Ok(Some(AtomKind::Aliphatic(Aliphatic::C))))
    }

    #[test]
    fn aromatic_carbon() {
        let mut scanner = Scanner::new("c");
        let atom = read_organic(&mut scanner);
        
        assert_eq!(atom, Ok(Some(AtomKind::Aromatic(Aromatic::C))))
    }

    #[test]
    fn chlorine() {
        let mut scanner = Scanner::new("Cl");
        let atom = read_organic(&mut scanner);
        
        assert_eq!(atom, Ok(Some(AtomKind::Aliphatic(Aliphatic::Cl))))
    }
}