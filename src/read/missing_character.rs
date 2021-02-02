use super::{ scanner::Scanner, Error };

pub fn missing_character(scanner: &mut Scanner) -> Error{
    if scanner.is_done() {
        Error::EndOfLine
    } else {
        Error::InvalidCharacter(scanner.cursor())
    }
}