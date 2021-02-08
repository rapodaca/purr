use crate::parts::{ AtomKind };
use super::Bond;

/// Atom used in graph-like (adjacency) SMILES representation.
#[derive(Debug,PartialEq)]
pub struct Atom {
    pub kind: AtomKind,
    pub bonds: Vec<Bond>
}

impl Atom {
    /// Consutrcts an Atom without bonds.
    pub fn new(kind: AtomKind) -> Self {
        Self {
            kind,
            bonds: vec![ ]
        }
    }

    /// Returns true if the atom was encoded as aromatic.
    pub fn is_aromatic(&self) -> bool {
        self.kind.is_aromatic()
    }

    /// Computes and returns the subvalence associated with this Atom.
    /// Subvalence represents the maximum number of [implicit hydrogens](https://depth-first.com/articles/2020/06/08/hydrogen-suppression-in-smiles/)
    /// that can be added to this Atom without exceeding a valence target.
    /// This value is independent of an atom's aromaticity marking.
    pub fn subvalence(&self) -> u8 {
        let hcount: u8 = match &self.kind {
            AtomKind::Bracket { hcount, .. } => match hcount {
                Some(hcount) => hcount.into(),
                None => 0
            }
            _ => 0
        };
        let valence = self.bonds.iter().fold(hcount, |sum,bond| {
            sum + bond.order()
        });
        let targets = self.kind.targets().iter()
            .find(|&&target| target >= valence);

        let target = match targets {
            Some(target) => target,
            None => return 0
        };
        
        target - valence
    }
}

#[cfg(test)]
mod subvalence {
    use crate::parts::{
        BondKind, BracketSymbol, Element, BracketAromatic, Aliphatic, Aromatic,
        Charge, VirtualHydrogen
    };
    use super::*;

    #[test]
    fn star() {
        let atom = Atom {
            kind: AtomKind::Star,
            bonds: vec![ ]
        };

        assert_eq!(atom.subvalence(), 0)
    }

    #[test]
    fn star_single() {
        let atom = Atom {
            kind: AtomKind::Star,
            bonds: vec![
                Bond::new(BondKind::Single, 1)
            ]
        };

        assert_eq!(atom.subvalence(), 0)
    }

    #[test]
    fn carbon_single() {
        let atom = Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            bonds: vec![
                Bond::new(BondKind::Single, 1)
            ]
        };

        assert_eq!(atom.subvalence(), 3)
    }

    #[test]
    fn aromatic_carbon_single() {
        let atom = Atom {
            kind: AtomKind::Aromatic(Aromatic::C),
            bonds: vec![
                Bond::new(BondKind::Single, 1)
            ]
        };

        assert_eq!(atom.subvalence(), 3)
    }

    #[test]
    fn bracket_star_single() {
        let atom = Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Star,
                configuration: None,
                hcount: None,
                charge: None,
                map: None
            },
            bonds: vec![
                Bond::new(BondKind::Single, 1)
            ]
        };

        assert_eq!(atom.subvalence(), 0)
    }

    #[test]
    fn bracket_carbon_h1() {
        let atom = Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Element(Element::C),
                configuration: None,
                hcount: Some(VirtualHydrogen::H1),
                charge: None,
                map: None
            },
            bonds: vec![

            ]
        };

        assert_eq!(atom.subvalence(), 3)
    }

    #[test]
    fn bracket_carbon_h0_single() {
        let atom = Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Element(Element::C),
                configuration: None,
                hcount: None,
                charge: None,
                map: None
            },
            bonds: vec![
                Bond::new(BondKind::Single, 1)
            ]
        };

        assert_eq!(atom.subvalence(), 3)
    }

    #[test]
    fn bracket_aromatic_h0_single() {
        let atom = Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Aromatic(BracketAromatic::C),
                configuration: None,
                hcount: None,
                charge: None,
                map: None
            },
            bonds: vec![
                Bond::new(BondKind::Single, 1)
            ]
        };

        assert_eq!(atom.subvalence(), 3)
    }

    #[test]
    fn bracket_aromatic_carbon_h1_single() {
        let atom = Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Aromatic(BracketAromatic::C),
                configuration: None,
                hcount: Some(VirtualHydrogen::H1),
                charge: None,
                map: None
            },
            bonds: vec![
                Bond::new(BondKind::Single, 1)
            ]
        };

        assert_eq!(atom.subvalence(), 2)
    }

    #[test]
    fn sulfur_charged_divalent() {
        let atom = Atom {
            kind: AtomKind::Bracket {
                isotope: None,
                symbol: BracketSymbol::Aromatic(BracketAromatic::S),
                configuration: None,
                hcount: None,
                charge: Some(Charge::One),
                map: None
            },
            bonds: vec![
                Bond::new(BondKind::Single, 1),
                Bond::new(BondKind::Single, 2)
            ]
        };

        assert_eq!(atom.subvalence(), 1)
    }
}