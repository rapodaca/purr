#[derive(Debug,PartialEq)]
pub enum Error {
    EndOfLine,
    InvalidCharacter(usize)
}