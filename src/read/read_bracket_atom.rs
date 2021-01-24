use crate::parts::{ AtomKind, Parity };
use crate::tree::{ Atom };
use super::{ Scanner, read_symbol, read_charge, Error };

pub fn read_bracket_atom(scanner: &mut Scanner) -> Result<Option<Atom>, Error> {
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

fn read_parity(scanner: &mut Scanner) -> Result<Option<Parity>, Error> {
    if let Some('@') = scanner.peek() {
        scanner.pop();

        if let Some('@') = scanner.peek() {
            scanner.pop();

            Ok(Some(Parity::Clockwise))
        } else {
            Ok(Some(Parity::Counterclockwise))
        }
    } else {
        Ok(None)
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