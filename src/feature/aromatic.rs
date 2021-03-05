use std::fmt;
use std::convert::TryFrom;

use super::{ Aliphatic, BracketAromatic };

/// Atomic symbols that can be aromatic.
#[derive(Debug,PartialEq)]
pub enum Aromatic {
    B, C, N, O, P, S
}

impl Aromatic {
    /// The valence targets available to this aromatic.
    pub fn targets(&self) -> &[u8] {
        match self {
            Self::B => &[ 3 ],
            Self::C => &[ 4 ],
            Self::N => &[ 3, 5 ],
            Self::O => &[ 2 ],
            Self::P => &[ 3, 5 ],
            Self::S => &[ 2, 4, 6 ]
        }
    }
}

impl TryFrom<&BracketAromatic> for Aromatic {
    type Error = ();

    fn try_from(value: &BracketAromatic) -> Result<Self, Self::Error> {
        match value {
            BracketAromatic::B => Ok(Self::B),
            BracketAromatic::C => Ok(Self::C),
            BracketAromatic::N => Ok(Self::N),
            BracketAromatic::O => Ok(Self::O),
            BracketAromatic::P => Ok(Self::P),
            BracketAromatic::S => Ok(Self::S),
            _ => Err(())
        }
    }
}

impl Into<Aliphatic> for &Aromatic {
    fn into(self) -> Aliphatic {
        match self {
            Aromatic::B => Aliphatic::B,
            Aromatic::C => Aliphatic::C,
            Aromatic::N => Aliphatic::N,
            Aromatic::O => Aliphatic::O,
            Aromatic::P => Aliphatic::P,
            Aromatic::S => Aliphatic::S
        }
    }
}

impl fmt::Display for Aromatic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::B => "b",
            Self::C => "c",
            Self::N => "n",
            Self::O => "o",
            Self::P => "p",
            Self::S => "s"
        })
    }
}