use std::convert::TryInto;

use crate::feature::Rnum;
use super::{ scanner::Scanner, Error, missing_character::missing_character };

pub fn read_rnum(
    scanner: &mut Scanner
) -> Result<Option<Rnum>, Error> {
    let mut digits = String::new();

    match scanner.peek() {
        Some('0'..='9') => {
            digits.push(*scanner.pop().unwrap());
        },
        Some('%') => {
            scanner.pop();

            for _ in 0..=1 {
                match scanner.peek() {
                    Some('0'..='9') => {
                        digits.push(*scanner.pop().expect("scanner done"));
                    },
                    _ => return Err(missing_character(scanner))
                }
            }
        },
        _ => return Ok(None)
    }

    let rnum = digits.parse::<u16>().expect("rnum to u16");

    Ok(Some(rnum.try_into().expect("u16 to rnum")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_digit() {
        let mut scanner = Scanner::new("%0");

        assert_eq!(read_rnum(&mut scanner), Err(Error::EndOfLine))
    }

    #[test]
    fn zero() {
        let mut scanner = Scanner::new("0");

        assert_eq!(read_rnum(&mut scanner), Ok(Some(Rnum::R0)))
    }

    #[test]
    fn nine() {
        let mut scanner = Scanner::new("9");

        assert_eq!(read_rnum(&mut scanner), Ok(Some(Rnum::R9)))
    }

    #[test]
    fn percent_zero_zero() {
        let mut scanner = Scanner::new("%00");

        assert_eq!(read_rnum(&mut scanner), Ok(Some(Rnum::R0)))
    }

    #[test]
    fn percent_nine_nine() {
        let mut scanner = Scanner::new("%99");

        assert_eq!(read_rnum(&mut scanner), Ok(Some(Rnum::R99)))
    }
}