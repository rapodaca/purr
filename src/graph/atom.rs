use crate::parts::{ AtomKind };
use super::Bond;

#[derive(Debug,PartialEq)]
pub struct Atom {
    pub kind: AtomKind,
    pub bonds: Vec<Bond>
}

impl Atom {
    pub fn new(kind: AtomKind) -> Self {
        Self {
            kind,
            bonds: vec![ ]
        }
    }

    pub fn is_aromatic(&self) -> bool {
        self.kind.is_aromatic()
    }

    pub fn subvalence(&self) -> u8 {
        let hcount = match &self.kind {
            AtomKind::Bracket { hcount, .. } =>
                hcount.unwrap_or_default(),
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
        BondKind, BracketSymbol, Element, BracketAromatic, Aliphatic, Aromatic
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
                parity: None,
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
                parity: None,
                hcount: Some(1),
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
                parity: None,
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
                parity: None,
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
                parity: None,
                hcount: Some(1),
                charge: None,
                map: None
            },
            bonds: vec![
                Bond::new(BondKind::Single, 1)
            ]
        };

        assert_eq!(atom.subvalence(), 2)
    }
}