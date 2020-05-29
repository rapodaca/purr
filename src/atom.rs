use crate::parity::Parity;
use crate::element::Element;

#[derive(PartialEq, Eq, Default, Debug, Clone, Copy)]
pub struct Atom {
    pub element: Element,
    pub aromatic: bool,
    pub isotope: Option<u16>,
    pub hcount: Option<u8>,
    pub charge: Option<i8>,
    pub parity: Option<Parity>,
    pub map: u16,
}