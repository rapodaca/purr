use super::{ BracketAromatic, Element };

#[derive(Debug,PartialEq)]
pub enum BracketSymbol {
    Star,
    Element(Element),
    Aromatic(BracketAromatic)
}