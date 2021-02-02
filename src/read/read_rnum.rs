use super::{ scanner::Scanner, Error, missing_character::missing_character };

pub fn read_rnum(
    scanner: &mut Scanner
) -> Result<Option<u16>, Error> {
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

    Ok(Some(digits.parse::<u16>().expect("rnum conversion")))
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

        assert_eq!(read_rnum(&mut scanner), Ok(Some(0)))
    }

    #[test]
    fn nine() {
        let mut scanner = Scanner::new("9");

        assert_eq!(read_rnum(&mut scanner), Ok(Some(9)))
    }

    #[test]
    fn percent_zero_zero() {
        let mut scanner = Scanner::new("%00");

        assert_eq!(read_rnum(&mut scanner), Ok(Some(0)))
    }

    #[test]
    fn percent_nine_nine() {
        let mut scanner = Scanner::new("%99");

        assert_eq!(read_rnum(&mut scanner), Ok(Some(99)))
    }
}