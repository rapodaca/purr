use std::fmt;
use std::convert::TryFrom;

/// Represents the virtual hydrogen count on a bracket atom.
/// See: [Hydrogen Suppression in SMILES](https://depth-first.com/articles/2020/06/08/hydrogen-suppression-in-smiles/).
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

impl TryFrom<u8> for VirtualHydrogen {
    type Error = ();

    fn try_from(count: u8) -> Result<Self, Self::Error> {
        match count {
            0 => Ok(Self::H0),
            1 => Ok(Self::H1),
            2 => Ok(Self::H2),
            3 => Ok(Self::H3),
            4 => Ok(Self::H4),
            5 => Ok(Self::H5),
            6 => Ok(Self::H6),
            7 => Ok(Self::H7),
            8 => Ok(Self::H8),
            9 => Ok(Self::H9),
            _ => Err(())
        }
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

impl fmt::Display for VirtualHydrogen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            VirtualHydrogen::H0 => "",
            VirtualHydrogen::H1 => "H",
            VirtualHydrogen::H2 => "H2",
            VirtualHydrogen::H3 => "H3",
            VirtualHydrogen::H4 => "H4",
            VirtualHydrogen::H5 => "H5",
            VirtualHydrogen::H6 => "H6",
            VirtualHydrogen::H7 => "H7",
            VirtualHydrogen::H8 => "H8",
            VirtualHydrogen::H9 => "H9",
        })
    }
}