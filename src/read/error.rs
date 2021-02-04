/// Syntax error resulting from scanning a SMILES string.
#[derive(Debug,PartialEq)]
pub enum Error {
    EndOfLine,
    InvalidCharacter(usize),
    NumberOverflow(usize)
}