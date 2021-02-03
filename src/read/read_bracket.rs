use crate::parts::{ AtomKind };
use crate::tree::{ Atom };
use super::{
    scanner::Scanner,
    read_symbol::read_symbol,
    read_charge::read_charge,
    read_parity::read_parity,
    Error
};

pub fn read_bracket(scanner: &mut Scanner) -> Result<Option<Atom>, Error> {
    if let Some('[') = scanner.peek() {
        scanner.pop();
    } else {
        return Ok(None);
    }

    let isotope = triple_digits(scanner);
    let symbol = read_symbol(scanner)?;
    let parity = read_parity(scanner)?;
    let hcount = read_hcount(scanner)?;
    let charge = read_charge(scanner)?;
    let map = read_map(scanner)?;

    match scanner.peek() {
        Some(']') => {
            scanner.pop();
            
            Ok(Some(Atom {
                kind: AtomKind::Bracket {
                    isotope, symbol, parity, hcount, charge, map
                },
                links: vec![ ]
            }))
        },
        None => Err(Error::EndOfLine),
        _ => Err(Error::InvalidCharacter(scanner.cursor()))
    }
}

fn read_hcount(scanner: &mut Scanner) -> Result<Option<u8>, Error> {
    if let Some('H') = scanner.peek() {
        scanner.pop();

        match scanner.peek() {
            Some('0'..='9') => {
                // https://stackoverflow.com/a/43985705/54426
                Ok(Some(*scanner.pop().unwrap() as u8 - '0' as u8))
            },
            Some(_) => {
                Ok(Some(1))
            },
            None => Ok(None)
        }
    } else {
        Ok(None)
    }
}



fn read_map(scanner: &mut Scanner) -> Result<Option<u16>, Error> {
    match scanner.peek() {
        Some(':') => {
            scanner.pop();

            match triple_digits(scanner) {
                Some(digits) => {
                    Ok(Some(digits))
                },
                None => Err(Error::InvalidCharacter(scanner.cursor()))
            }
        },
        _ => Ok(None)
    }
}

fn triple_digits(scanner: &mut Scanner) -> Option<u16> {
    let mut string = String::new();

    for _ in 0..3 {
        if let Some(digit) = scanner.peek() {
            if digit.is_ascii_digit() {
                string.push(*scanner.pop().unwrap());
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if string.is_empty() {
        None
    } else {
        Some(string.parse::<u16>().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::parts::{ BracketSymbol, Parity, BracketAromatic, Charge };
    use crate::tree::Atom;
    use super::*;

    #[test]
    fn bracket_invalid() {
        let mut scanner = Scanner::new("[Q]");

        assert_eq!(read_bracket(&mut scanner), Err(Error::InvalidCharacter(1)))
    }

    #[test]
    fn no_close() {
        let mut scanner = Scanner::new("[C");

        assert_eq!(read_bracket(&mut scanner), Err(Error::EndOfLine))
    }

    #[test]
    fn no_open() {
        let mut scanner = Scanner::new("?");

        assert_eq!(read_bracket(&mut scanner), Ok(None))
    }

    #[test]
    fn star() {
        let mut scanner = Scanner::new("[*]");

        assert_eq!(read_bracket(&mut scanner), Ok(Some(Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Star,
                parity: None,
                hcount: None,
                charge: None,
                map: None
            },
            links: vec![ ]
        })))
    }

    #[test]
    fn star_isotope() {
        let mut scanner = Scanner::new("[12*]");

        assert_eq!(read_bracket(&mut scanner), Ok(Some(Atom {
            kind: AtomKind::Bracket {
                isotope: Some(12),
                symbol: BracketSymbol::Star,
                parity: None,
                hcount: None,
                charge: None,
                map: None
            },
            links: vec![ ]
        })))
    }

    #[test]
    fn star_parity() {
        let mut scanner = Scanner::new("[*@]");

        assert_eq!(read_bracket(&mut scanner), Ok(Some(Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Star,
                parity: Some(Parity::Counterclockwise),
                hcount: None,
                charge: None,
                map: None
            },
            links: vec![ ]
        })))
    }

    #[test]
    fn star_hcount() {
        let mut scanner = Scanner::new("[*H2]");

        assert_eq!(read_bracket(&mut scanner), Ok(Some(Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Star,
                parity: None,
                hcount: Some(2),
                charge: None,
                map: None
            },
            links: vec![ ]
        })))
    }

    #[test]
    fn star_charge() {
        let mut scanner = Scanner::new("[*+]");

        assert_eq!(read_bracket(&mut scanner), Ok(Some(Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Star,
                parity: None,
                hcount: None,
                charge: Some(Charge::One),
                map: None
            },
            links: vec![ ]
        })))
    }

    #[test]
    fn star_map() {
        let mut scanner = Scanner::new("[*:123]");

        assert_eq!(read_bracket(&mut scanner), Ok(Some(Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Star,
                parity: None,
                hcount: None,
                charge: None,
                map: Some(123)
            },
            links: vec![ ]
        })))
    }

    #[test]
    fn bracket_aromatic_charge() {
        let mut scanner = Scanner::new("[s+]");

        assert_eq!(read_bracket(&mut scanner), Ok(Some(Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Aromatic(BracketAromatic::S),
                parity: None,
                hcount: None,
                charge: Some(Charge::One),
                map: None
            },
            links: vec![ ]
        })))
    }
}