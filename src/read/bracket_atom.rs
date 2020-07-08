use crate::mol::Nub;
use crate::mol::Parity;
use super::error::Error;
use super::symbol::symbol;
use crate::util::Scanner;

pub fn bracket_atom(scanner: &mut Scanner) -> Result<Option<Nub>, Error> {
    if let Some('[') = scanner.peek() {
        scanner.pop();
    } else {
        return Ok(None);
    }

    let mut atom = Nub {
        hcount: Some(0), charge: Some(0), ..Default::default()
    };

    atom.isotope = triple_digits(scanner);

    symbol(scanner, &mut atom)?;
    chiral(scanner, &mut atom);
    hcount(scanner, &mut atom);
    charge(scanner, &mut atom);
    map(scanner, &mut atom)?;

    match scanner.peek() {
        Some(']') => {
            scanner.pop();
            
            Ok(Some(atom))
        },
        None => Err(Error::EndOfLine),
        _ => Err(Error::InvalidCharacter(scanner.cursor()))
    }
}

fn chiral(scanner: &mut Scanner, atom: &mut Nub) {
    if let Some('@') = scanner.peek() {
        scanner.pop();

        if let Some('@') = scanner.peek() {
            scanner.pop();

            atom.parity = Some(Parity::Clockwise);
        } else {
            atom.parity = Some(Parity::Counterclockwise);
        }
    }
}

fn hcount(scanner: &mut Scanner, atom: &mut Nub) {
    if let Some('H') = scanner.peek() {
        scanner.pop();

        match scanner.peek() {
            Some('0'..='9') => {
                // https://stackoverflow.com/a/43985705/54426
                atom.hcount = Some(*scanner.pop().unwrap() as u8 - '0' as u8);
            },
            Some(_) => {
                atom.hcount = Some(1);
            },
            None => { }
        }
    }
}

fn charge(scanner: &mut Scanner, atom: &mut Nub) {
    match scanner.peek() {
        Some('+') => {
            scanner.pop();

            match fifteen(scanner) {
                Some(value) => {
                    atom.charge = Some(value);
                },
                None => {
                    match scanner.peek() {
                        Some('+') => {
                            scanner.pop();

                            atom.charge = Some(2);
                        },
                        _ => {
                            atom.charge = Some(1);
                        }
                    }
                }
            }
        },
        Some('-') => {
            scanner.pop();

            match fifteen(scanner) {
                Some(value) => {
                    atom.charge = Some(-value);
                },
                None => {
                    match scanner.peek() {
                        Some('-') => {
                            scanner.pop();

                            atom.charge = Some(-2);
                        }
                        _ => {
                            atom.charge = Some(-1);
                        }
                    }
                }
            }
        },
        _ => ()
    }
}

fn map(scanner: &mut Scanner, atom: &mut Nub) -> Result<(), Error> {
    match scanner.peek() {
        Some(':') => {
            scanner.pop();

            match triple_digits(scanner) {
                Some(digits) => {
                    atom.map = digits;

                    Ok(())
                },
                None => Err(Error::InvalidCharacter(scanner.cursor()))
            }
        },
        _ => Ok(())
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

fn fifteen(scanner: &mut Scanner) -> Option<i8> {
    match scanner.peek() {
        Some('1') => {
            scanner.pop();

            match scanner.peek() {
                Some('0') => { scanner.pop(); Some(10) },
                Some('1') => { scanner.pop(); Some(11) },
                Some('2') => { scanner.pop(); Some(12) },
                Some('3') => { scanner.pop(); Some(13) },
                Some('4') => { scanner.pop(); Some(14) },
                Some('5') => { scanner.pop(); Some(15) },
                _ => Some(1)
            }
        },
        Some('2'..='9') => Some(*scanner.pop().unwrap() as i8 - '0' as i8),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mol::Element;

    #[test]
    fn no_leading_bracket() {
        let mut scanner = Scanner::new(&"X");
        let result = bracket_atom(&mut scanner);

        assert_eq!(result, Ok(None));
    }

    #[test]
    fn no_trailing_bracket_with_char() {
        let mut scanner = Scanner::new(&"[CX");
        let result = bracket_atom(&mut scanner);

        assert_eq!(result, Err(Error::InvalidCharacter(2)));
    }

    #[test]
    fn no_traling_bracket_eol() {
        let mut scanner = Scanner::new(&"[C");
        let result = bracket_atom(&mut scanner);

        assert_eq!(result, Err(Error::EndOfLine));
    }

    #[test]
    fn unknown_symbol() {
        let mut scanner = Scanner::new(&"[J]");
        let result = bracket_atom(&mut scanner);

        assert_eq!(result, Err(Error::InvalidCharacter(1)));
    }

    #[test]
    fn isotope_given_four_digits() {
        let mut scanner = Scanner::new(&"[1234C]");
        let result = bracket_atom(&mut scanner);

        assert_eq!(result, Err(Error::InvalidCharacter(4)));
    }

    #[test]
    fn charge_plus_zero() {
        let mut scanner = Scanner::new(&"[C+0]");
        let result = bracket_atom(&mut scanner);

        assert_eq!(result, Err(Error::InvalidCharacter(3)));
    }

    #[test]
    fn charge_with_plus_minus() {
        let mut scanner = Scanner::new(&"[C+-]");
        let result = bracket_atom(&mut scanner);

        assert_eq!(result, Err(Error::InvalidCharacter(3)));
    }

    #[test]
    fn charge_with_minus_zero() {
        let mut scanner = Scanner::new(&"[C-0]");
        let result = bracket_atom(&mut scanner);

        assert_eq!(result, Err(Error::InvalidCharacter(3)));
    }

    #[test]
    fn charge_with_minus_plus() {
        let mut scanner = Scanner::new(&"[C-+]");
        let result = bracket_atom(&mut scanner);

        assert_eq!(result, Err(Error::InvalidCharacter(3)));
    }

    #[test]
    fn carbon() {
        let mut scanner = Scanner::new(&"[C]");
        let result = bracket_atom(&mut scanner).unwrap();

        assert_eq!(result, Some(Nub {
            hcount: Some(0), charge: Some(0), ..Default::default()
        }));
    }

    #[test]
    fn aromatic_carbon() {
        let mut scanner = Scanner::new(&"[c]");
        let result = bracket_atom(&mut scanner).unwrap();

        assert_eq!(result, Some(Nub {
            hcount: Some(0), aromatic: true,
            charge: Some(0), ..Default::default()
        }));
    }

    #[test]
    fn isotope() {
        let mut scanner = Scanner::new(&"[13C]");
        let result = bracket_atom(&mut scanner).unwrap();

        assert_eq!(result, Some(Nub {
            hcount: Some(0), charge: Some(0), isotope: Some(13),
            ..Default::default()
        }));
    }

    #[test]
    fn isotope_given_leading_zero() {
        let mut scanner = Scanner::new(&"[013C]");
        let result = bracket_atom(&mut scanner).unwrap();

        assert_eq!(result, Some(Nub {
            hcount: Some(0), isotope: Some(13), charge: Some(0), 
            ..Default::default()
        }));
    }

    #[test]
    fn hcount_without_digit() {
        let mut scanner = Scanner::new(&"[CH]");
        let result = bracket_atom(&mut scanner).unwrap();

        assert_eq!(result, Some(Nub {
            hcount: Some(1), charge: Some(0), ..Default::default()
        }));
    }

    #[test]
    fn hcount_with_digit() {
        let mut scanner = Scanner::new(&"[CH2]");
        let result = bracket_atom(&mut scanner).unwrap();

        assert_eq!(result, Some(Nub {
            hcount: Some(2), charge: Some(0), ..Default::default()
        }));
    }

    #[test]
    fn charge_with_plus() {
        let mut scanner = Scanner::new(&"[C+]");
        let result = bracket_atom(&mut scanner).unwrap();

        assert_eq!(result, Some(Nub {
            hcount: Some(0), charge: Some(1), ..Default::default()
        }));
    }

    #[test]
    fn charge_with_plus_one() {
        let mut scanner = Scanner::new(&"[C+1]");
        let result = bracket_atom(&mut scanner).unwrap();

        assert_eq!(result, Some(Nub {
            hcount: Some(0), charge: Some(1), ..Default::default()
        }));
    }

    #[test]
    fn charge_with_plus_two() {
        let mut scanner = Scanner::new(&"[Ca+2]");
        let result = bracket_atom(&mut scanner).unwrap();
    
        assert_eq!(result, Some(Nub {
            hcount: Some(0), element: Element::Ca,
            charge: Some(2), ..Default::default()
        }));
    }

    #[test]
    fn charge_with_plus_plus() {
        let mut scanner = Scanner::new(&"[Ca++]");
        let result = bracket_atom(&mut scanner).unwrap();
    
        assert_eq!(result, Some(Nub {
            hcount: Some(0), element: Element::Ca,
            charge: Some(2), ..Default::default()
        }));
    }

    #[test]
    fn charge_with_plus_fifteen() {
        let mut scanner = Scanner::new(&"[Zn+15]");
        let result = bracket_atom(&mut scanner).unwrap();
    
        assert_eq!(result, Some(Nub {
            hcount: Some(0), element: Element::Zn,
            charge: Some(15), ..Default::default()
        }));
    }

    #[test]
    fn charge_with_minus() {
        let mut scanner = Scanner::new(&"[Cl-]");
        let result = bracket_atom(&mut scanner).unwrap();

        assert_eq!(result, Some(Nub {
            hcount: Some(0), element: Element::Cl,
            charge: Some(-1), ..Default::default()
        }));
    }

    #[test]
    fn charge_with_minus_one() {
        let mut scanner = Scanner::new(&"[Cl-1]");
        let result = bracket_atom(&mut scanner).unwrap();

        assert_eq!(result, Some(Nub {
            hcount: Some(0), element: Element::Cl,
            charge: Some(-1), ..Default::default()
        }));
    }

    #[test]
    fn charge_with_minus_two() {
        let mut scanner = Scanner::new(&"[O-2]");
        let result = bracket_atom(&mut scanner).unwrap();

        assert_eq!(result, Some(Nub {
            hcount: Some(0), element: Element::O,
            charge: Some(-2), ..Default::default()
        }));
    }

    #[test]
    fn charge_with_minus_minus() {
        let mut scanner = Scanner::new(&"[O--]");
        let result = bracket_atom(&mut scanner).unwrap();

        assert_eq!(result, Some(Nub {
            hcount: Some(0), element: Element::O,
            charge: Some(-2), ..Default::default()
        }));
    }

    #[test]
    fn charge_with_minus_fifteen() {
        let mut scanner = Scanner::new(&"[Ti-15]");
        let result = bracket_atom(&mut scanner).unwrap();
    
        assert_eq!(result, Some(Nub {
            hcount: Some(0), element: Element::Ti,
            charge: Some(-15), ..Default::default()
        }));
    }

    #[test]
    fn map_with_zero_zero_zero() {
        let mut scanner = Scanner::new(&"[C:000]");
        let result = bracket_atom(&mut scanner).unwrap();
    
        assert_eq!(result, Some(Nub {
            hcount: Some(0), charge: Some(0),  ..Default::default()
        }));
    }

    #[test]
    fn map_with_nine_nine_nine() {
        let mut scanner = Scanner::new(&"[C:999]");
        let result = bracket_atom(&mut scanner).unwrap();
    
        assert_eq!(result, Some(Nub {
            hcount: Some(0), charge: Some(0), map: 999, ..Default::default()
        }));
    }

    #[test]
    fn kitchen_sink() {
        let mut scanner = Scanner::new(&"[15n@H+:123]");
        let result = bracket_atom(&mut scanner).unwrap();
    
        assert_eq!(result, Some(Nub {
            isotope: Some(15), element: Element::N, aromatic: true,
            hcount: Some(1), charge: Some(1), map: 123,
            parity: Some(Parity::Counterclockwise)
        }));
        assert_eq!(scanner.done(), true);
    }
}