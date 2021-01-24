use super::Element;

#[derive(Debug,PartialEq)]
pub enum BracketAromatic {
    B, C, N, O, S, P, Se, As
}

impl Into<Element> for &BracketAromatic {
    fn into(self) -> Element {
        match self {
            BracketAromatic::As => Element::As,
            BracketAromatic::B => Element::B,
            BracketAromatic::C => Element::C,
            BracketAromatic::N => Element::N,
            BracketAromatic::O => Element::O,
            BracketAromatic::P => Element::P,
            BracketAromatic::S => Element::S,
            BracketAromatic::Se => Element::Se
        }
    }
}