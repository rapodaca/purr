use super::atom::Atom;
use super::bond::Bond;

#[derive(PartialEq, Eq, Default, Debug)]
pub struct Mol {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Vec<Bond>>
}