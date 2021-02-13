use std::fmt;

use super::Aliphatic;

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