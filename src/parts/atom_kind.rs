use super::{
    Aliphatic, Aromatic, BracketSymbol, Parity, Element, Charge, VirtualHydrogen
};

/// Minimal context-sensitive representation of an atom kind.
#[derive(Debug,PartialEq)]
pub enum AtomKind {
    Star,
    Aliphatic(Aliphatic),
    Aromatic(Aromatic),
    Bracket {
        isotope: Option<u16>,
        symbol: BracketSymbol,
        parity: Option<Parity>,
        hcount: Option<VirtualHydrogen>,
        charge: Option<Charge>,
        map: Option<u16>
    }
}

impl AtomKind {
    /// Returns true if the kind was defined as being aromatic.
    pub fn is_aromatic(&self) -> bool {
        match self {
            Self::Star => false,
            Self::Aromatic(_) => true,
            Self::Aliphatic(_) => false,
            Self::Bracket { symbol, .. } => match symbol {
                BracketSymbol::Aromatic(_) => true,
                BracketSymbol::Element(_) => false,
                BracketSymbol::Star => false
            }
        }
    }

    /// Returns the valence targets for this atom kind.
    pub fn targets(&self) -> &[u8] {
        match self {
            Self::Star => &[ ],
            Self::Aliphatic(aliphatic) => aliphatic.targets(),
            Self::Aromatic(aromatic) => aromatic.targets(),
            Self::Bracket { symbol, charge, .. } => match symbol {
                BracketSymbol::Star => &[ ],
                BracketSymbol::Aromatic(aromatic) => 
                    elemental_targets(&aromatic.into(), charge),
                BracketSymbol::Element(element) =>
                    elemental_targets(element, charge)
            }
        }
    }
}

fn elemental_targets(element: &Element, charge: &Option<Charge>) -> &'static [u8] {
    match element {
        Element::B => match charge {
            Some(Charge::MinusThree) => &OXYGEN_TARGET,
            Some(Charge::MinusTwo) => &NITROGEN_TARGET,
            Some(Charge::MinusOne) => &CARBON_TARGET,
            Some(Charge::Zero) |
            None => &BORON_TARGET,
            _ => &EMPTY_TARGET
        },
        Element::C => match charge {
            Some(Charge::MinusTwo) => &OXYGEN_TARGET,
            Some(Charge::MinusOne) => &NITROGEN_TARGET,
            Some(Charge::Zero) |
            Some(Charge::One) => &BORON_TARGET,
            None => &CARBON_TARGET,
            _ => &EMPTY_TARGET
        },
        Element::N => match charge {
            Some(Charge::Zero) |
            None => &NITROGEN_TARGET,
            Some(Charge::One) => &CARBON_TARGET,
            _ => &EMPTY_TARGET
        },
        Element::O => match charge {
            Some(Charge::Zero) |
            None => &OXYGEN_TARGET,
            Some(Charge::One) => &NITROGEN_TARGET,
            _ => &EMPTY_TARGET
        },
        Element::P |
        Element::As => match charge {
            Some(Charge::MinusOne) => &SULFUR_TARGET,
            Some(Charge::Zero) |
            None => &PHOSPHOROUS_TARGET,
            _ => &EMPTY_TARGET
        },
        Element::S |
        Element::Se => match charge {
            Some(Charge::Zero) |
            None => &SULFUR_TARGET,
            Some(Charge::One) => &PHOSPHOROUS_TARGET,
            _ => &EMPTY_TARGET
        }
        _ => &EMPTY_TARGET
    }
}

static BORON_TARGET: [u8; 1] = [ 3 ];
static CARBON_TARGET: [u8; 1] = [ 4 ];
static NITROGEN_TARGET: [u8; 2] = [ 3, 5 ];
static OXYGEN_TARGET: [u8; 1] = [ 2 ];
static PHOSPHOROUS_TARGET: [u8; 2] = [ 3, 5 ];
static SULFUR_TARGET: [u8; 3] = [ 2, 4, 6 ];
static EMPTY_TARGET: [u8; 0] = [ ];