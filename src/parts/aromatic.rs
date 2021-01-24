use super::Aliphatic;

#[derive(Debug,PartialEq)]
pub enum Aromatic {
    B, C, N, O, P, S
}

impl Aromatic {
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