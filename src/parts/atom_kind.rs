use super::{ Aliphatic, Aromatic, BracketSymbol, Parity, Element };

#[derive(Debug,PartialEq)]
pub enum AtomKind {
    Star,
    Aliphatic(Aliphatic),
    Aromatic(Aromatic),
    Bracket {
        isotope: Option<u16>,
        symbol: BracketSymbol,
        parity: Option<Parity>,
        hcount: Option<u8>,
        charge: Option<i8>,
        map: Option<u16>
    }
}

impl AtomKind {
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

fn elemental_targets(element: &Element, charge: &Option<i8>) -> &'static [u8] {
    match element {
        Element::B => match charge {
            Some(-3) => &OXYGEN_TARGET,
            Some(-2) => &NITROGEN_TARGET,
            Some(-1) => &CARBON_TARGET,
            Some(0) |
            None => &BORON_TARGET,
            _ => &EMPTY_TARGET
        },
        Element::C => match charge {
            Some(-2) => &OXYGEN_TARGET,
            Some(-1) => &NITROGEN_TARGET,
            Some(0) |
            Some(1) => &BORON_TARGET,
            None => &CARBON_TARGET,
            _ => &EMPTY_TARGET
        },
        Element::N => match charge {
            Some(0) |
            None => &NITROGEN_TARGET,
            Some(1) => &CARBON_TARGET,
            _ => &EMPTY_TARGET
        },
        Element::O => match charge {
            Some(0) |
            None => &OXYGEN_TARGET,
            Some(1) => &NITROGEN_TARGET,
            _ => &EMPTY_TARGET
        },
        Element::P |
        Element::As => match charge {
            Some(-1) => &SULFUR_TARGET,
            Some(0) |
            None => &PHOSPHOROUS_TARGET,
            _ => &EMPTY_TARGET
        },
        Element::S |
        Element::Se => match charge {
            Some(0) |
            None => &SULFUR_TARGET,
            Some(1) => &PHOSPHOROUS_TARGET,
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