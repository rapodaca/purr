use std::convert::Into;

/// Represents the virtual hydrogen count on a bracket atom. See: [Hydrogen Suppression in SMILES](https://depth-first.com/articles/2020/06/08/hydrogen-suppression-in-smiles/).
#[derive(Debug,PartialEq)]
pub enum VirtualHydrogen {
    H0,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
    H9
}

impl VirtualHydrogen {
    /// Returns true if the count is zero, or false otherwise.
    pub fn is_zero(&self) -> bool {
        self == &VirtualHydrogen::H0
    }
}

impl Into<u8> for &VirtualHydrogen {
    fn into(self) -> u8 {
        match self {
            VirtualHydrogen::H0 => 0,
            VirtualHydrogen::H1 => 1,
            VirtualHydrogen::H2 => 2,
            VirtualHydrogen::H3 => 3,
            VirtualHydrogen::H4 => 4,
            VirtualHydrogen::H5 => 5,
            VirtualHydrogen::H6 => 6,
            VirtualHydrogen::H7 => 7,
            VirtualHydrogen::H8 => 8,
            VirtualHydrogen::H9 => 9
        }
    }
}