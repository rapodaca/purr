use std::convert::TryFrom;
use std::fmt;

use super::{
    Aliphatic, Aromatic, BracketSymbol, Configuration, Element, Charge,
    VirtualHydrogen, Number
};

/// Minimal context-sensitive representation of an atom kind.
#[derive(Debug,PartialEq)]
pub enum AtomKind {
    Star,
    Aliphatic(Aliphatic),
    Aromatic(Aromatic),
    Bracket {
        isotope: Option<Number>,
        symbol: BracketSymbol,
        configuration: Option<Configuration>,
        hcount: Option<VirtualHydrogen>,
        charge: Option<Charge>,
        map: Option<Number>
    }
}

impl AtomKind {
    /// Returns an unbracketed version of this AtomKind based on
    /// `bond_order_sum`, if possible. Already unbracketed AtomKinds return
    /// self.
    /// 
    /// This method is intended for clients building representations from
    /// outside sources. It allows for a single, always valid bracketed AtomKind
    /// to be constructed and debracketed, if possible. The logic to decide
    /// debracketability is encapsulated here.
    pub fn debracket(self, bond_order_sum: u8) -> AtomKind {
        let (isotope, symbol, configuration, hcount ,charge, map) = match &self {
            AtomKind::Star => return self,
            AtomKind::Aliphatic(_) => return self,
            AtomKind::Aromatic(_) => return self,
            AtomKind::Bracket {
                isotope, symbol, configuration, hcount, charge, map
            } => (isotope, symbol, configuration, hcount, charge, map)
        };

        if any(isotope, configuration, charge, map) {
            return self
        }

        match symbol {
            BracketSymbol::Star => match hcount {
                Some(hcount) => {
                    if hcount.is_zero() {
                        AtomKind::Star
                    } else {
                        self
                    }
                },
                None => AtomKind::Star
            },
            BracketSymbol::Aromatic(aromatic) => {
                let hcount = match hcount {
                    Some(hcount) => hcount.into(),
                    None => 0
                };
                let valence = bond_order_sum.checked_add(hcount)
                    .expect("valence");
                let allowance = if hcount == 0 { 0 } else { 1 };
                let aromatic = match Aromatic::try_from(aromatic) {
                    Ok(aromatic) => aromatic,
                    Err(_) => return self
                };

                for target in aromatic.targets() {
                    if valence == target - allowance {
                        return AtomKind::Aromatic(aromatic)
                    }
                }

                self
            },
            BracketSymbol::Element(element) => {
                let valence = bond_order_sum.checked_add(match hcount {
                    Some(hcount) => hcount.into(),
                    None => 0
                }).expect("valence");
                let aliphatic = match Aliphatic::try_from(element) {
                    Ok(aliphatic) => aliphatic,
                    Err(_) => return self
                };

                for target in aliphatic.targets() {
                    if target == &valence {
                        return AtomKind::Aliphatic(aliphatic)
                    }
                }

                self
            }
        }
    }

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

    /// Inverts configuration given if it and at least one implicit
    /// hydrogen are present.
    /// 
    /// # Panics
    /// 
    /// Panics given a Configuration other than TH1 or TH2.
    pub fn invert_configuration(&mut self) {
        if let AtomKind::Bracket { hcount, configuration, .. } = self {
            let new_config = match configuration {
                Some(config) => match hcount {
                    Some(hcount) => if hcount.is_zero() {
                        return
                    } else {
                        match config {
                            Configuration::TH1 => Configuration::TH2,
                            Configuration::TH2 => Configuration::TH1,
                            _ => unimplemented!("TODO: handle inversion for non-TH")
                        }
                    },
                    None => return
                },
                None => return
            };
    
            configuration.replace(new_config);
        }
    }
}

fn any(
    isotope: &Option<Number>,
    configuration: &Option<Configuration>,
    charge: &Option<Charge>,
    map: &Option<Number>
) -> bool {
    isotope.is_some() || configuration.is_some() || charge.is_some()
        || map.is_some()
}

fn elemental_targets(element: &Element, charge: &Option<Charge>) -> &'static [u8] {
    match element {
        Element::B => match charge {
            Some(Charge::MinusThree) => &OXYGEN_TARGET,
            Some(Charge::MinusTwo) => &NITROGEN_TARGET,
            Some(Charge::MinusOne) => &CARBON_TARGET,
            None => &BORON_TARGET,
            _ => &EMPTY_TARGET
        },
        Element::C => match charge {
            Some(Charge::MinusTwo) => &OXYGEN_TARGET,
            Some(Charge::MinusOne) => &NITROGEN_TARGET,
            Some(Charge::One) => &BORON_TARGET,
            None => &CARBON_TARGET,
            _ => &EMPTY_TARGET
        },
        Element::N => match charge {
            None => &NITROGEN_TARGET,
            Some(Charge::One) => &CARBON_TARGET,
            _ => &EMPTY_TARGET
        },
        Element::O => match charge {
            None => &OXYGEN_TARGET,
            Some(Charge::One) => &NITROGEN_TARGET,
            _ => &EMPTY_TARGET
        },
        Element::P |
        Element::As => match charge {
            Some(Charge::MinusOne) => &SULFUR_TARGET,
            None => &PHOSPHOROUS_TARGET,
            _ => &EMPTY_TARGET
        },
        Element::S |
        Element::Se => match charge {
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

impl fmt::Display for AtomKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AtomKind::Star => write!(f, "*"),
            AtomKind::Aliphatic(aliphatic) => write!(f, "{}", aliphatic),
            AtomKind::Aromatic(aromatic) => write!(f, "{}", aromatic),
            AtomKind::Bracket {
                isotope, symbol, hcount, configuration, charge, map
            } => {
                write!(f, "[")?;

                if let Some(isotope) = isotope {
                    write!(f, "{}", isotope)?
                }

                write!(f, "{}", symbol)?;
                
                if let Some(configuration) = configuration {
                    write!(f, "{}", configuration)?
                }

                if let Some(hcount) = hcount {
                    write!(f, "{}", hcount)?
                }

                if let Some(charge) = charge {
                    write!(f, "{}", charge)?
                }

                if let Some(map) = map {
                    write!(f, ":{}", map)?
                }

                write!(f, "]")
            }
        }
    }
}

#[cfg(test)]
mod invert {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn aliphatic_organic() {
        let mut kind = AtomKind::Aliphatic(Aliphatic::C);

        kind.invert_configuration();

        match kind {
            AtomKind::Aliphatic(_) => (),
            _ => panic!("expected aliphatic")
        }
    }

    #[test]
    fn th1_h_none() {
        let mut kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            configuration: Some(Configuration::TH1),
            hcount: None,
            charge: None,
            map: None
        };

        kind.invert_configuration();

        match kind {
            AtomKind::Bracket { configuration, .. } =>
                assert_eq!(configuration, Some(Configuration::TH1)),
            _ => panic!("expected bracket")
        }
    }

    #[test]
    fn th1_h1() {
        let mut kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            configuration: Some(Configuration::TH1),
            hcount: Some(VirtualHydrogen::H1),
            charge: None,
            map: None
        };

        kind.invert_configuration();

        match kind {
            AtomKind::Bracket { configuration, .. } =>
                assert_eq!(configuration, Some(Configuration::TH2)),
            _ => panic!("expected bracket")
        }
    }

    #[test]
    fn th2_h1() {
        let mut kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            configuration: Some(Configuration::TH2),
            hcount: Some(VirtualHydrogen::H1),
            charge: None,
            map: None
        };

        kind.invert_configuration();

        match kind {
            AtomKind::Bracket { configuration, .. } =>
                assert_eq!(configuration, Some(Configuration::TH1)),
            _ => panic!("expected bracket")
        }
    }
}