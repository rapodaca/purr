use crate::mol::Error as MolError;

#[derive(PartialEq, Debug)]
pub enum Error {
    EndOfLine,
    InvalidCharacter(usize),
    MolError(usize, MolError)
}