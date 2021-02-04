use super::{ scanner::Scanner, Error };

pub fn read_number(scanner: &mut Scanner) -> Result<Option<u16>, Error> {
    let cursor = scanner.cursor();
    let mut string = String::new();

    while let Some(digit) = scanner.peek() {
        if digit.is_ascii_digit() {
            string.push(*scanner.pop().expect("digit"))
        } else {
            break;
        }
    }

    if string.is_empty() {
        Ok(None)
    } else {
        match string.parse::<u16>() {
            Ok(number) => Ok(Some(number)),
            Err(_) => Err(Error::NumberOverflow(cursor))
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn overflow() {
        let mut scanner = Scanner::new("65536");

        assert_eq!(read_number(&mut scanner), Err(Error::NumberOverflow(0)))
    }

    #[test]
    fn leading_non_digit() {
        let mut scanner = Scanner::new("x123");

        assert_eq!(read_number(&mut scanner), Ok(None))
    }

    #[test]
    fn leading_zero() {
        let mut scanner = Scanner::new("042");

        assert_eq!(read_number(&mut scanner), Ok(Some(42)))
    }

    #[test]
    fn character_termination() {
        let mut scanner = Scanner::new("42x");

        assert_eq!(read_number(&mut scanner), Ok(Some(42)))
    }

    #[test]
    fn max_value() {
        let mut scanner = Scanner::new("65535");

        assert_eq!(read_number(&mut scanner), Ok(Some(u16::MAX)))
    }
}