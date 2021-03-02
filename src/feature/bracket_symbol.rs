use std::fmt;

use super::{ BracketAromatic, Element };

/// Represents those atomic symbols capable of appearing within a bracket
/// atom in the string representation.
#[derive(Debug,PartialEq)]
pub enum BracketSymbol {
    Star,
    Element(Element),
    Aromatic(BracketAromatic)
}

impl fmt::Display for BracketSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BracketSymbol::Star => write!(f, "*"),
            BracketSymbol::Aromatic(aromatic) => write!(f, "{}", aromatic),
            BracketSymbol::Element(element) => write!(f, "{}", element)
        }
    }
}