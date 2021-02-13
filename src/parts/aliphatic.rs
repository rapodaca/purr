use std::fmt;

/// Atomic symbols that can be aliphatic.
#[derive(Debug,PartialEq)]
pub enum Aliphatic {
    B, C, N, O, S, P, F, Cl, Br, I, At, Ts
}

impl Aliphatic {
    pub fn targets(&self) -> &[u8] {
        match self {
            Self::B => &[ 3 ],
            Self::C => &[ 4 ],
            Self::N |
            Self::P => &[ 3, 5 ],
            Self::O => &[ 2 ],
            Self::S => &[ 2, 4, 6 ],
            Self::F |
            Self::Cl |
            Self::Br |
            Self::I |
            Self::At |
            Self::Ts => &[ 1 ]
        }
    }
}

impl fmt::Display for Aliphatic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::B => "B",
            Self::C => "C",
            Self::N => "N",
            Self::O => "O",
            Self::S => "S",
            Self::P => "P",
            Self::F => "F",
            Self::Cl => "Cl",
            Self::Br => "Br",
            Self::I => "I",
            Self::At => "At",
            Self::Ts => "Ts"
        })
    }
}