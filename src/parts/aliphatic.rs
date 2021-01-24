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