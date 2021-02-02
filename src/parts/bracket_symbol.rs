use super::{ BracketAromatic, Element };

/// Represents those atomic symbols capable of appearing within a bracket
/// atom in the string representation.
#[derive(Debug,PartialEq)]
pub enum BracketSymbol {
    Star,
    Element(Element),
    Aromatic(BracketAromatic)
}