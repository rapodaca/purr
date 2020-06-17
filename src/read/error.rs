#[derive(PartialEq, Debug)]
pub enum Error {
    EndOfLine,
    InvalidCharacter(usize),
    MismatchedStyle,
    InvalidState
}