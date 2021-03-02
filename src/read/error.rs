/// An error that occurs when reading a SMILES string.
#[derive(Debug,PartialEq)]
pub enum Error {
    EndOfLine,
    Character(usize)
}