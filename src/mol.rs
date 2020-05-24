use crate::Atom;
use crate::Bond;

#[derive(PartialEq, Eq, Default, Debug)]
pub struct Mol {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Vec<Bond>>
}