use std::convert::TryInto;

use crate::feature::Charge;
use super::{ scanner::Scanner, Error };

pub fn read_charge(scanner: &mut Scanner) -> Result<Option<Charge>, Error> {
    match scanner.peek() {
        Some('+') => {
            scanner.pop();

            match fifteen(scanner) {
                Some(value) => Ok(Some(value.try_into().expect("charge"))),
                None => match scanner.peek() {
                    Some('+') => {
                        scanner.pop();

                        Ok(Some(Charge::Two))
                    },
                    _ => Ok(Some(Charge::One))
                }
            }
        },
        Some('-') => {
            scanner.pop();

            match fifteen(scanner) {
                Some(value) => Ok(Some((-value).try_into().expect("charge"))),
                None => match scanner.peek() {
                    Some('-') => {
                        scanner.pop();

                        Ok(Some(Charge::MinusTwo))
                    }
                    _ => Ok(Some(Charge::MinusOne))
                }
            }
        },
        _ => Ok(None)
    }
}

fn fifteen(scanner: &mut Scanner) -> Option<i8> {
    match scanner.peek() {
        Some('1'..='9') => Some(match scanner.pop() {
            Some('1') => match scanner.peek() {
                Some('1'..='5') => match scanner.pop() {
                    Some('1') => 11,
                    Some('2') => 12,
                    Some('3') => 13,
                    Some('4') => 14,
                    Some('5') => 15,
                    _ => 1
                },
                _ => 1
            },
            Some('2') => 2,
            Some('3') => 3,
            Some('4') => 4,
            Some('5') => 5,
            Some('6') => 6,
            Some('7') => 7,
            Some('8') => 8,
            Some('9') => 9,
            _ => unreachable!("fifteen")
        }),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn none() {
        let mut scanner = Scanner::new("X");

        assert_eq!(read_charge(&mut scanner), Ok(None))
    }

    #[test]
    fn minus_x() {
        let mut scanner = Scanner::new("-X");

        assert_eq!(read_charge(&mut scanner), Ok(Some(Charge::MinusOne)))
    }

    #[test]
    fn minus_2_x() {
        let mut scanner = Scanner::new("-1X");

        assert_eq!(read_charge(&mut scanner), Ok(Some(Charge::MinusOne)))
    }

    #[test]
    fn minus_minus_x() {
        let mut scanner = Scanner::new("--X");

        assert_eq!(read_charge(&mut scanner), Ok(Some(Charge::MinusTwo)))
    }

    #[test]
    fn minus_15_x() {
        let mut scanner = Scanner::new("-15X");

        assert_eq!(read_charge(&mut scanner), Ok(Some(Charge::MinusFifteen)))
    }

    #[test]
    fn plus_x() {
        let mut scanner = Scanner::new("+X");

        assert_eq!(read_charge(&mut scanner), Ok(Some(Charge::One)))
    }

    #[test]
    fn plus_plus_x() {
        let mut scanner = Scanner::new("++X");

        assert_eq!(read_charge(&mut scanner), Ok(Some(Charge::Two)))
    }

    #[test]
    fn plus_2_x() {
        let mut scanner = Scanner::new("+2X");

        assert_eq!(read_charge(&mut scanner), Ok(Some(Charge::Two)))
    }

    #[test]
    fn plus_15_x() {
        let mut scanner = Scanner::new("+15X");

        assert_eq!(read_charge(&mut scanner), Ok(Some(Charge::Fifteen)))
    }
}