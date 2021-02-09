use crate::parts::{
    AtomKind, Aliphatic, Aromatic, BracketSymbol, BracketAromatic, Configuration,
    Element, Charge, VirtualHydrogen, Number
};

pub fn write_atom_kind(kind: &AtomKind, out: &mut String) {
    match &kind {
        AtomKind::Star => write_star(out),
        AtomKind::Aliphatic(aliphatic) => write_aliphatic(aliphatic, out),
        AtomKind::Aromatic(aromatic) => write_aromatic(aromatic, out),
        AtomKind::Bracket {
            isotope, symbol, hcount, charge, configuration, map
        } => write_bracket(
            isotope, symbol, hcount, charge, configuration, map, out
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
    isotope: &Option<Number>,
    symbol: &BracketSymbol,
    hcount: &Option<VirtualHydrogen>,
    charge: &Option<Charge>,
    configuration: &Option<Configuration>,
    map: &Option<Number>,
    out: &mut String
) {
    out.push('[');

    write_isotope(isotope, out);
    write_symbol(symbol, out);
    write_configuration(configuration, out);
    write_hcount(hcount, out);
    write_charge(charge, out);
    write_map(map, out);

    out.push(']')
}

fn write_isotope(isotope: &Option<Number>, out: &mut String) {
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

fn write_configuration(
    configuration: &Option<Configuration>, out: &mut String
) {
    match configuration {
        Some(Configuration::TH1) => out.push_str("@"),
        Some(Configuration::TH2) => out.push_str("@@"),
        Some(Configuration::AL1) => out.push_str("@AL1"),
        Some(Configuration::AL2) => out.push_str("@AL2"),
        Some(Configuration::TB1) => out.push_str("@TB1"),
        Some(Configuration::TB2) => out.push_str("@TB2"),
        Some(Configuration::TB3) => out.push_str("@TB3"),
        Some(Configuration::TB4) => out.push_str("@TB4"),
        Some(Configuration::TB5) => out.push_str("@TB5"),
        Some(Configuration::TB6) => out.push_str("@TB6"),
        Some(Configuration::TB7) => out.push_str("@TB7"),
        Some(Configuration::TB8) => out.push_str("@TB8"),
        Some(Configuration::TB9) => out.push_str("@TB9"),
        Some(Configuration::TB10) => out.push_str("@TB10"),
        Some(Configuration::TB11) => out.push_str("@TB11"),
        Some(Configuration::TB12) => out.push_str("@TB12"),
        Some(Configuration::TB13) => out.push_str("@TB13"),
        Some(Configuration::TB14) => out.push_str("@TB14"),
        Some(Configuration::TB15) => out.push_str("@TB15"),
        Some(Configuration::TB16) => out.push_str("@TB16"),
        Some(Configuration::TB17) => out.push_str("@TB17"),
        Some(Configuration::TB18) => out.push_str("@TB18"),
        Some(Configuration::TB19) => out.push_str("@TB19"),
        Some(Configuration::TB20) => out.push_str("@TH1"),
        Some(Configuration::OH1) => out.push_str("@OH1"),
        Some(Configuration::OH2) => out.push_str("@OH2"),
        Some(Configuration::OH3) => out.push_str("@TH1"),
        Some(Configuration::OH4) => out.push_str("@OH4"),
        Some(Configuration::OH5) => out.push_str("@OH5"),
        Some(Configuration::OH6) => out.push_str("@OH6"),
        Some(Configuration::OH7) => out.push_str("@OH7"),
        Some(Configuration::OH8) => out.push_str("@OH8"),
        Some(Configuration::OH9) => out.push_str("@OH9"),
        Some(Configuration::OH10) => out.push_str("@OH10"),
        Some(Configuration::OH11) => out.push_str("@OH11"),
        Some(Configuration::OH12) => out.push_str("@OH12"),
        Some(Configuration::OH13) => out.push_str("@OH13"),
        Some(Configuration::OH14) => out.push_str("@OH6"),
        Some(Configuration::OH15) => out.push_str("@OH15"),
        Some(Configuration::OH16) => out.push_str("@OH16"),
        Some(Configuration::OH17) => out.push_str("@OH17"),
        Some(Configuration::OH18) => out.push_str("@OH18"),
        Some(Configuration::OH19) => out.push_str("@OH19"),
        Some(Configuration::OH20) => out.push_str("@OH20"),
        Some(Configuration::OH21) => out.push_str("@OH21"),
        Some(Configuration::OH22) => out.push_str("@OH22"),
        Some(Configuration::OH23) => out.push_str("@OH23"),
        Some(Configuration::OH24) => out.push_str("@OH24"),
        Some(Configuration::OH25) => out.push_str("@OH25"),
        Some(Configuration::OH26) => out.push_str("@OH26"),
        Some(Configuration::OH27) => out.push_str("@OH27"),
        Some(Configuration::OH28) => out.push_str("@OH28"),
        Some(Configuration::OH29) => out.push_str("@OH29"),
        Some(Configuration::OH30) => out.push_str("@OH30"),
        Some(Configuration::SP1) => out.push_str("@SP1"),
        Some(Configuration::SP2) => out.push_str("@SP2"),
        Some(Configuration::SP3) => out.push_str("@SP3"),
        None => (),
    }
}

fn write_hcount(hcount: &Option<VirtualHydrogen>, out: &mut String) {
    out.push_str(match hcount {
        Some(VirtualHydrogen::H0) => return,
        Some(VirtualHydrogen::H1) => "H",
        Some(VirtualHydrogen::H2) => "H2",
        Some(VirtualHydrogen::H3) => "H3",
        Some(VirtualHydrogen::H4) => "H4",
        Some(VirtualHydrogen::H5) => "H5",
        Some(VirtualHydrogen::H6) => "H6",
        Some(VirtualHydrogen::H7) => "H7",
        Some(VirtualHydrogen::H8) => "H8",
        Some(VirtualHydrogen::H9) => "H9",
        _ => return
    })
}

fn write_charge(charge: &Option<Charge>, out: &mut String) {
    match charge {
        Some(charge) => {
            out.push_str(match charge {
                Charge::MinusFifteen => "-15",
                Charge::MinusFourteen => "-14",
                Charge::MinusThirteen => "-13",
                Charge::MinusTwelve => "-12",
                Charge::MinusEleven => "-11",
                Charge::MinusTen => "-10",
                Charge::MinusNine => "-9",
                Charge::MinusEight => "-8",
                Charge::MinusSeven => "-7",
                Charge::MinusSix => "-6",
                Charge::MinusFive => "-5",
                Charge::MinusFour => "-4",
                Charge::MinusThree => "-3",
                Charge::MinusTwo => "-2",
                Charge::MinusOne => "-",
                Charge::Zero => "0",
                Charge::One => "+",
                Charge::Two => "+2",
                Charge::Three => "+3",
                Charge::Four => "+4",
                Charge::Five =>" +5",
                Charge::Six => "+6",
                Charge::Seven => "+7",
                Charge::Eight =>"+8",
                Charge::Nine => "+9",
                Charge::Ten => "+10",
                Charge::Eleven => "+11",
                Charge::Twelve => "+12",
                Charge::Thirteen => "+13",
                Charge::Fourteen => "+14",
                Charge::Fifteen => "+15"
            })
        },
        None => ()
    }
}

fn write_map(map: &Option<Number>, out: &mut String) {
    if let Some(map) = map {
        out.push(':');
        out.push_str(&map.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;
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
            configuration: None,
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
            configuration: None,
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
            configuration: None,
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
            isotope: Some(12.try_into().unwrap()),
            symbol: BracketSymbol::Star,
            configuration: None,
            hcount: None,
            charge: None,
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[12*]")
    }

    #[test]
    fn configuration_clockwise() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            configuration: Some(Configuration::TH2),
            hcount: None,
            charge: None,
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*@@]")
    }

    #[test]
    fn configuration_counterclockwise() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            configuration: Some(Configuration::TH1),
            hcount: None,
            charge: None,
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*@]")
    }

    #[test]
    fn configuration_tetrahedral() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            configuration: Some(Configuration::TH1),
            hcount: None,
            charge: None,
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*@]")
    }

    #[test]
    fn hcount_zero() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            configuration: None,
            hcount: Some(VirtualHydrogen::H0),
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
            configuration: None,
            hcount: Some(VirtualHydrogen::H1),
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
            configuration: None,
            hcount: Some(VirtualHydrogen::H2),
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
            configuration: None,
            hcount: None,
            charge: Some(Charge::Zero),
            map: None
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*0]")
    }

    #[test]
    fn charge_plus_one() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            configuration: None,
            hcount: None,
            charge: Some(Charge::One),
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
            configuration: None,
            hcount: None,
            charge: Some(Charge::Two),
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
            configuration: None,
            hcount: None,
            charge: Some(Charge::MinusOne),
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
            configuration: None,
            hcount: None,
            charge: Some(Charge::MinusTwo),
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
            configuration: None,
            hcount: None,
            charge: None,
            map: Some(0.try_into().unwrap())
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*:0]")
    }

    #[test]
    fn map_thirteen() {
        let kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
            configuration: None,
            hcount: None,
            charge: None,
            map: Some(13.try_into().unwrap())
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[*:13]")
    }

    #[test]
    fn kitchen_sink() {
        let kind = AtomKind::Bracket {
            isotope: Some(13.try_into().unwrap()),
            symbol: BracketSymbol::Element(Element::C),
            configuration: Some(Configuration::TH2),
            hcount: Some(VirtualHydrogen::H1),
            charge: Some(Charge::One),
            map: Some(42.try_into().unwrap())
        };
        let mut out = String::new();

        write_atom_kind(&kind, &mut out);

        assert_eq!(out, "[13C@@H+:42]")
    }
}