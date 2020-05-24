#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    EndOfLine,
    InvalidCharacter(usize),
    MismatchedStyle
}