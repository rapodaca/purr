use super::{ Scanner, Error };

pub fn read_charge(scanner: &mut Scanner) -> Result<Option<i8>, Error> {
    match scanner.peek() {
        Some('+') => {
            scanner.pop();

            match fifteen(scanner) {
                Some(value) => Ok(Some(value)),
                None => match scanner.peek() {
                    Some('+') => {
                        scanner.pop();

                        Ok(Some(2))
                    },
                    _ => Ok(Some(1))
                }
            }
        },
        Some('-') => {
            scanner.pop();

            match fifteen(scanner) {
                Some(value) => Ok(Some(-value)),
                None => match scanner.peek() {
                    Some('-') => {
                        scanner.pop();

                        Ok(Some(-2))
                    }
                    _ => Ok(Some(-1))
                }
            }
        },
        _ => Ok(None)
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