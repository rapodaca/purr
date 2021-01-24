use crate::parts::BondKind;
use super::Scanner;

pub fn read_bond(scanner: &mut Scanner) -> BondKind {
    let result = match scanner.peek() {
        Some('-') => BondKind::Single,
        Some('=') => BondKind::Double,
        Some('#') => BondKind::Triple,
        Some('$') => BondKind::Quadruple,
        Some(':') => BondKind::Aromatic,
        Some('/') => BondKind::Up,
        Some('\\') => BondKind::Down,
        _ => BondKind::Elided
    };

    if result != BondKind::Elided {
        scanner.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single() {
        let mut scanner = Scanner::new("-");

        assert_eq!(read_bond(&mut scanner), BondKind::Single);
    }

    #[test]
    fn double() {
        let mut scanner = Scanner::new("=");

        assert_eq!(read_bond(&mut scanner), BondKind::Double);
    }

    #[test]
    fn triple() {
        let mut scanner = Scanner::new("#");

        assert_eq!(read_bond(&mut scanner), BondKind::Triple);
    }

    #[test]
    fn quadruple() {
        let mut scanner = Scanner::new("$");

        assert_eq!(read_bond(&mut scanner), BondKind::Quadruple);
    }

    #[test]
    fn aromatic() {
        let mut scanner = Scanner::new(":");

        assert_eq!(read_bond(&mut scanner), BondKind::Aromatic);
    }

    #[test]
    fn up() {
        let mut scanner = Scanner::new("/");

        assert_eq!(read_bond(&mut scanner), BondKind::Up);
    }

    #[test]
    fn down() {
        let mut scanner = Scanner::new("\\");

        assert_eq!(read_bond(&mut scanner), BondKind::Down);
    }
}