use crate::parts::{
    AtomKind, Aliphatic, Aromatic, BracketSymbol, BracketAromatic, Parity,
    Element
};

pub fn write_atom_kind(kind: &AtomKind, out: &mut String) {
    match &kind {
        AtomKind::Star => write_star(out),
        AtomKind::Aliphatic(aliphatic) => write_aliphatic(aliphatic, out),
        AtomKind::Aromatic(aromatic) => write_aromatic(aromatic, out),
        AtomKind::Bracket {
            isotope, symbol, hcount, charge, parity, map
        } => write_bracket(
            isotope, symbol, hcount, charge, parity, map, out
        )
    }
}

fn write_star(out: &mut String) {
    out.push('*')
}

fn write_aliphatic(aliphatic: &Aliphatic, out: &mut String) {
    out.push_str(match aliphatic {
        Aliphatic::At => "At",
        Aliphatic::B =>  "B",
        Aliphatic::Br => "Br",
        Aliphatic::C =>  "C",
        Aliphatic::Cl => "Cl",
        Aliphatic::F =>  "F",
        Aliphatic::I =>  "I",
        Aliphatic::N =>  "N",
        Aliphatic::O =>  "O",
        Aliphatic::P =>  "P",
        Aliphatic::S =>  "S",
        Aliphatic::Ts => "Ts"
    })
}

fn write_aromatic(aromatic: &Aromatic, out: &mut String) {
    out.push_str(match aromatic {
        Aromatic::B => "b",
        Aromatic::C => "c",
        Aromatic::N => "n",
        Aromatic::O => "o",
        Aromatic::P => "p",
        Aromatic::S => "s"
    })
}

fn write_bracket(
    isotope: &Option<u16>,
    symbol: &BracketSymbol,
    hcount: &Option<u8>,
    charge: &Option<i8>,
    parity: &Option<Parity>,
    map: &Option<u16>,
    out: &mut String
) {
    out.push('[');

    write_isotope(isotope, out);
    write_symbol(symbol, out);
    write_parity(parity, out);
    write_hcount(hcount, out);
    write_charge(charge, out);
    write_map(map, out);

    out.push(']')
}

fn write_isotope(isotope: &Option<u16>, out: &mut String) {
    if let Some(isotope) = isotope {
        out.push_str(&isotope.to_string())
    }
}

fn write_symbol(symbol: &BracketSymbol, out: &mut String) {
    match symbol {
        BracketSymbol::Star => out.push('*'),
        BracketSymbol::Aromatic(aromatic) =>
            write_bracket_aromatic(aromatic, out),
        BracketSymbol::Element(element) => write_element(element, out)
    }
}

fn write_bracket_aromatic(aromatic: &BracketAromatic, out: &mut String) {
    out.push_str(match aromatic {
        BracketAromatic::B => "b",
        BracketAromatic::C => "c",
        BracketAromatic::N => "n",
        BracketAromatic::O => "o",
        BracketAromatic::S => "s",
        BracketAromatic::P => "p",
        BracketAromatic::Se => "se",
        BracketAromatic::As => "as"
    })
}

fn write_element(element: &Element, out: &mut String) {
    out.push_str(match element {
        Element::Ac => "Ac",
        Element::Ag => "Ag",
        Element::Al => "Al",
        Element::Am => "Am",
        Element::Ar => "Ar",
        Element::As => "As",
        Element::At => "At",
        Element::Au => "Au",
        Element::B  => "B",
        Element::Ba => "Ba",
        Element::Be => "Be",
        Element::Bh => "Bh",
        Element::Bi => "Bi",
        Element::Bk => "Bk",
        Element::Br => "Br",
        Element::C  => "C",
        Element::Ca => "Ca",
        Element::Cd => "Cd",
        Element::Ce => "Ce",
        Element::Cf => "Cf",
        Element::Cl => "Cl",
        Element::Cm => "Cm",
        Element::Cn => "Cn",
        Element::Co => "Co",
        Element::Cr => "Cr",
        Element::Cs => "Ac",
        Element::Cu => "Cu",
        Element::Db => "Db",
        Element::Ds => "Ds",
        Element::Dy => "Dy",
        Element::Er => "Er",
        Element::Es => "Es",
        Element::Eu => "Eu",
        Element::F  => "F",
        Element::Fe => "Fe",
        Element::Fl => "Fl",
        Element::Fm => "Fm",
        Element::Fr => "Fr",
        Element::Ga => "Ga",
        Element::Gd => "Gd",
        Element::Ge => "Ge",
        Element::H  => "H",
        Element::He => "He",
        Element::Hf => "Hf",
        Element::Hg => "Hg",
        Element::Ho => "Ho",
        Element::Hs => "Hs",
        Element::I  => "I",
        Element::In => "In",
        Element::Ir => "Ir",
        Element::K  => "K",
        Element::Kr => "Kr",
        Element::La => "La",
        Element::Li => "Li",
        Element::Lr => "Lr",
        Element::Lu => "Lu",
        Element::Lv => "Lv",
        Element::Mc => "Mc",
        Element::Md => "Md",
        Element::Mg => "Mg",
        Element::Mn => "Mn",
        Element::Mo => "Mo",
        Element::Mt => "Mt",
        Element::Na => "Na",
        Element::Nb => "Nb",
        Element::Nd => "Nd",
        Element::N  => "N",
        Element::Ne => "Ne",
        Element::Nh => "Nh",
        Element::Ni => "Ni",
        Element::No => "No",
        Element::Np => "Np",
        Element::O  => "O",
        Element::Os => "Os",
        Element::Og => "Og",
        Element::P  => "P",
        Element::Pa => "Pa",
        Element::Pb => "Pb",
        Element::Pd => "Pd",
        Element::Pm => "Pm",
        Element::Po => "Po",
        Element::Pr => "Pr",
        Element::Pt => "Pt",
        Element::Pu => "Pu",
        Element::Ra => "Ra",
        Element::Rb => "Rb",
        Element::Re => "Re",
        Element::Rf => "Rf",
        Element::Rg => "Rg",
        Element::Rh => "Rh",
        Element::Rn => "Rn",
        Element::Ru => "Ru",
        Element::S  => "S",
        Element::Sb => "Sb",
        Element::Sc => "Sc",
        Element::Se => "Se",
        Element::Sg => "Sg",
        Element::Si => "Si",
        Element::Sm => "Sm",
        Element::Sn => "Sn",
        Element::Sr => "Sr",
        Element::Ta => "Ta",
        Element::Tb => "Tb",
        Element::Tc => "Tc",
        Element::Te => "Te",
        Element::Th => "Th",
        Element::Ti => "Ti",
        Element::Tl => "Tl",
        Element::Tm => "Tm",
        Element::Ts => "Ts",
        Element::U  => "U",
        Element::V  => "V",
        Element::W  => "W",
        Element::Xe => "Xe",
        Element::Y  => "Y",
        Element::Yb => "Yb",
        Element::Zn => "Zn",
        Element::Zr => "Zr"
    })
}

fn write_parity(parity: &Option<Parity>, out: &mut String) {
    match parity {
        Some(Parity::Clockwise) => out.push_str("@@"),
        Some(Parity::Counterclockwise) => out.push('@'),
        Some(Parity::TH1) => out.push_str("@TH1"),
        Some(Parity::TH2) => out.push_str("@TH2"),
        Some(Parity::AL1) => out.push_str("@AL1"),
        Some(Parity::AL2) => out.push_str("@AL2"),
        Some(Parity::TB1) => out.push_str("@TB1"),
        Some(Parity::TB2) => out.push_str("@TB2"),
        Some(Parity::TB3) => out.push_str("@TB3"),
        Some(Parity::TB4) => out.push_str("@TB4"),
        Some(Parity::TB5) => out.push_str("@TB5"),
        Some(Parity::TB6) => out.push_str("@TB6"),
        Some(Parity::TB7) => out.push_str("@TB7"),
        Some(Parity::TB8) => out.push_str("@TB8"),
        Some(Parity::TB9) => out.push_str("@TB9"),
        Some(Parity::TB10) => out.push_str("@TB10"),
        Some(Parity::TB11) => out.push_str("@TB11"),
        Some(Parity::TB12) => out.push_str("@TB12"),
        Some(Parity::TB13) => out.push_str("@TB13"),
        Some(Parity::TB14) => out.push_str("@TB14"),
        Some(Parity::TB15) => out.push_str("@TB15"),
        Some(Parity::TB16) => out.push_str("@TB16"),
        Some(Parity::TB17) => out.push_str("@TB17"),
        Some(Parity::TB18) => out.push_str("@TB18"),
        Some(Parity::TB19) => out.push_str("@TB19"),
        Some(Parity::TB20) => out.push_str("@TH1"),
        Some(Parity::OH1) => out.push_str("@OH1"),
        Some(Parity::OH2) => out.push_str("@OH2"),
        Some(Parity::OH3) => out.push_str("@TH1"),
        Some(Parity::OH4) => out.push_str("@OH4"),
        Some(Parity::OH5) => out.push_str("@OH5"),
        Some(Parity::OH6) => out.push_str("@OH6"),
        Some(Parity::OH7) => out.push_str("@OH7"),
        Some(Parity::OH8) => out.push_str("@OH8"),
        Some(Parity::OH9) => out.push_str("@OH9"),
        Some(Parity::OH10) => out.push_str("@OH10"),
        Some(Parity::OH11) => out.push_str("@OH11"),
        Some(Parity::OH12) => out.push_str("@OH12"),
        Some(Parity::OH13) => out.push_str("@OH13"),
        Some(Parity::OH14) => out.push_str("@OH6"),
        Some(Parity::OH15) => out.push_str("@OH15"),
        Some(Parity::OH16) => out.push_str("@OH16"),
        Some(Parity::OH17) => out.push_str("@OH17"),
        Some(Parity::OH18) => out.push_str("@OH18"),
        Some(Parity::OH19) => out.push_str("@OH19"),
        Some(Parity::OH20) => out.push_str("@OH20"),
        Some(Parity::OH21) => out.push_str("@OH21"),
        Some(Parity::OH22) => out.push_str("@OH22"),
        Some(Parity::OH23) => out.push_str("@OH23"),
        Some(Parity::OH24) => out.push_str("@OH24"),
        Some(Parity::OH25) => out.push_str("@OH25"),
        Some(Parity::OH26) => out.push_str("@OH26"),
        Some(Parity::OH27) => out.push_str("@OH27"),
        Some(Parity::OH28) => out.push_str("@OH28"),
        Some(Parity::OH29) => out.push_str("@OH29"),
        Some(Parity::OH30) => out.push_str("@OH30"),
        Some(Parity::SP1) => out.push_str("@SP1"),
        Some(Parity::SP2) => out.push_str("@SP2"),
        Some(Parity::SP3) => out.push_str("@SP3"),
        None => (),
    }
}

fn write_hcount(hcount: &Option<u8>, out: &mut String) {
    if let Some(hcount) = *hcount {
        if hcount > 0 {
            out.push('H');

            if hcount > 1 {
                out.push_str(&hcount.to_string())
            }
        }
    }
}

fn write_charge(charge: &Option<i8>, out: &mut String) {
    if let Some(charge) = *charge {
        if charge > 0 {
            out.push('+');

            if charge > 1 {
                out.push_str(&charge.to_string())
            }
        } else if charge < 0 {
            if charge == -1 {
                out.push('-')
            } else {
                out.push_str(&charge.to_string())
            }
        }
    }
}

fn write_map(map: &Option<u16>, out: &mut String) {
    if let Some(map) = *map {
        if map > 0 {
            out.push(':');
            out.push_str(&map.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn star() {
        let kind = AtomKind::Star;
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "*")
    }

    #[test]
    fn aliphatic() {
        let kind = AtomKind::Aliphatic(Aliphatic::C);
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "C")
    }

    #[test]
    fn aromatic() {
        let kind = AtomKind::Aromatic(Aromatic::C);
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "c")
    }

    #[test]
    fn bracket_star() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            parity: None,
            hcount: None,
            charge: None,
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*]")
    }

    #[test]
    fn bracket_aromatic() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Aromatic(BracketAromatic::C),
            parity: None,
            hcount: None,
            charge: None,
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[c]")
    }

    #[test]
    fn bracket_element() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Element(Element::C),
            parity: None,
            hcount: None,
            charge: None,
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[C]")
    }

    #[test]
    fn isotope() {
        let kind = AtomKind::Bracket {
            isotope: Some(12),
            symbol: BracketSymbol::Star,
            parity: None,
            hcount: None,
            charge: None,
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[12*]")
    }

    #[test]
    fn parity_clockwise() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            parity: Some(Parity::Clockwise),
            hcount: None,
            charge: None,
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*@@]")
    }

    #[test]
    fn parity_counterclockwise() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            parity: Some(Parity::Counterclockwise),
            hcount: None,
            charge: None,
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*@]")
    }

    #[test]
    fn parity_tetrahedral() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            parity: Some(Parity::TH1),
            hcount: None,
            charge: None,
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*@TH1]")
    }

    #[test]
    fn hcount_zero() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            parity: None,
            hcount: Some(0),
            charge: None,
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*]")
    }

    #[test]
    fn hcount_one() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            parity: None,
            hcount: Some(1),
            charge: None,
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*H]")
    }

    #[test]
    fn hcount_two() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            parity: None,
            hcount: Some(2),
            charge: None,
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*H2]")
    }

    #[test]
    fn charge_zero() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            parity: None,
            hcount: None,
            charge: Some(0),
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*]")
    }

    #[test]
    fn charge_plus_one() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            parity: None,
            hcount: None,
            charge: Some(1),
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*+]")
    }

    #[test]
    fn charge_plus_two() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            parity: None,
            hcount: None,
            charge: Some(2),
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*+2]")
    }

    #[test]
    fn charge_minus_one() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            parity: None,
            hcount: None,
            charge: Some(-1),
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*-]")
    }

    #[test]
    fn charge_minus_two() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            parity: None,
            hcount: None,
            charge: Some(-2),
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*-2]")
    }

    #[test]
    fn map_zero() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            parity: None,
            hcount: None,
            charge: None,
            map: Some(0)
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*]")
    }

    #[test]
    fn map_thirteen() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            parity: None,
            hcount: None,
            charge: None,
            map: Some(13)
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*:13]")
    }

    #[test]
    fn kitchen_sink() {
        let kind = AtomKind::Bracket {
            isotope: Some(13),
            symbol: BracketSymbol::Element(Element::C),
            parity: Some(Parity::Clockwise),
            hcount: Some(1),
            charge: Some(1),
            map: Some(42)
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[13C@@H+:42]")
    }
}