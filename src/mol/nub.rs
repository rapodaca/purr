use super::Element;
use super::Parity;

#[derive(PartialEq, Eq, Default, Debug, Clone, Copy)]
pub struct Nub {
    pub element: Element,
    pub aromatic: bool,
    pub isotope: Option<u16>,
    pub hcount: Option<u8>,
    pub charge: Option<i8>,
    pub parity: Option<Parity>,
    pub map: u16,
}