use super::Nub;
use super::Bond;

#[derive(PartialEq, Eq, Default, Debug)]
pub struct Atom {
    pub nub: Nub,
    pub bonds: Vec<Bond>
}

impl Atom {
    pub fn new(nub: Nub) -> Self {
        Self {
            nub, bonds: vec![ ]
        }
    }
}