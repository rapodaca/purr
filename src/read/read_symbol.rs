use crate::parts::{ Element, BracketSymbol, BracketAromatic };
use super::{ Scanner, Error, missing_character };

pub fn read_symbol(scanner: &mut Scanner) -> Result<BracketSymbol, Error> {
    match scanner.peek() {
        Some('*') => {
            scanner.pop();

            Ok(BracketSymbol::Star)
        },
        Some('a') => {
            scanner.pop();

            match scanner.peek() {
                Some('s') => aromatic(BracketAromatic::As, scanner),
                _ => Err(missing_character(scanner))
            }
        },
        Some('b') => aromatic(BracketAromatic::B, scanner),
        Some('c') => aromatic(BracketAromatic::C, scanner),
        Some('n') => aromatic(BracketAromatic::N, scanner),
        Some('o') => aromatic(BracketAromatic::O, scanner),
        Some('p') => aromatic(BracketAromatic::P, scanner),
        Some('s') => {
            scanner.pop();

            match scanner.peek() {
                Some('e') => aromatic(BracketAromatic::Se, scanner),
                _ => aromatic(BracketAromatic::S, scanner)
            }
        },
        Some('A') => {
            scanner.pop();

            match scanner.peek() {
                Some('c') => element(Element::Ac, scanner),
                Some('g') => element(Element::Ag, scanner),
                Some('l') => element(Element::Al, scanner),
                Some('m') => element(Element::Am, scanner),
                Some('r') => element(Element::Ar, scanner),
                Some('s') => element(Element::As, scanner),
                Some('t') => element(Element::At, scanner),
                Some('u') => element(Element::Au, scanner),
                _ => Err(missing_character(scanner))
            }
        },
        Some('B') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => element(Element::Ba, scanner),
                Some('e') => element(Element::Be, scanner),
                Some('h') => element(Element::Bh, scanner),
                Some('i') => element(Element::Bi, scanner),
                Some('k') => element(Element::Bk, scanner),
                Some('r') => element(Element::Br, scanner),
                _ => Ok(BracketSymbol::Element(Element::B))
            }
        },
        Some('C') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => element(Element::Ca, scanner),
                Some('d') => element(Element::Cd, scanner),
                Some('e') => element(Element::Ce, scanner),
                Some('f') => element(Element::Cf, scanner),
                Some('l') => element(Element::Cl, scanner),
                Some('m') => element(Element::Cm, scanner),
                Some('n') => element(Element::Cn, scanner),
                Some('o') => element(Element::Co, scanner),
                Some('r') => element(Element::Cr, scanner),
                Some('s') => element(Element::Cs, scanner),
                Some('u') => element(Element::Cu, scanner),
                _ => Ok(BracketSymbol::Element(Element::C))
            }
        },
        Some('D') => {
            scanner.pop();

            match scanner.peek() {
                Some('b') => element(Element::Db, scanner),
                Some('s') => element(Element::Ds, scanner),
                Some('y') => element(Element::Dy, scanner),
                _ => Err(missing_character(scanner))
            }
        },
        Some('E') => {
            scanner.pop();

            match scanner.peek() {
                Some('r') => element(Element::Er, scanner),
                Some('s') => element(Element::Es, scanner),
                Some('u') => element(Element::Eu, scanner),
                _ => Err(missing_character(scanner))
            }
        },
        Some('F') => {
            scanner.pop();

            match scanner.peek() {
                Some('e') => element(Element::Fe, scanner),
                Some('l') => element(Element::Fl, scanner),
                Some('m') => element(Element::Fm, scanner),
                Some('r') => element(Element::Fr, scanner),
                _ => Ok(BracketSymbol::Element(Element::F))
            }
        },
        Some('G') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => element(Element::Ga, scanner),
                Some('d') => element(Element::Gd, scanner),
                Some('e') => element(Element::Ge, scanner),
                _ => Ok(BracketSymbol::Element(Element::F))
            }
        },
        Some('H') => {
            scanner.pop();

            match scanner.peek() {
                Some('e') => element(Element::He, scanner),
                Some('f') => element(Element::Hf, scanner),
                Some('g') => element(Element::Hg, scanner),
                Some('o') => element(Element::Ho, scanner),
                Some('s') => element(Element::Hs, scanner),
                _ => Ok(BracketSymbol::Element(Element::H))
            }
        },
        Some('I') => {
            scanner.pop();

            match scanner.peek() {
                Some('n') => element(Element::In, scanner),
                Some('r') => element(Element::Ir, scanner),
                _ => Ok(BracketSymbol::Element(Element::I))
            }
        },
        Some('K') => {
            scanner.pop();

            match scanner.peek() {
                Some('r') => element(Element::Kr, scanner),
                _ => Ok(BracketSymbol::Element(Element::K))
            }
        },
        Some('L') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => element(Element::La, scanner),
                Some('i') => element(Element::Li, scanner),
                Some('r') => element(Element::Lr, scanner),
                Some('u') => element(Element::Lu, scanner),
                Some('v') => element(Element::Lv, scanner),
                _ => Err(missing_character(scanner))
            }
        },
        Some('M') => {
            scanner.pop();

            match scanner.peek() {
                Some('c') => element(Element::Mc, scanner),
                Some('d') => element(Element::Md, scanner),
                Some('g') => element(Element::Mg, scanner),
                Some('n') => element(Element::Mn, scanner),
                Some('o') => element(Element::Mo, scanner),
                Some('t') => element(Element::Mt, scanner),
                _ => Err(missing_character(scanner))
            }
        },
        Some('N') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => element(Element::Na, scanner),
                Some('b') => element(Element::Nb, scanner),
                Some('d') => element(Element::Nd, scanner),
                Some('e') => element(Element::Ne, scanner),
                Some('h') => element(Element::Nh, scanner),
                Some('i') => element(Element::Ni, scanner),
                Some('o') => element(Element::No, scanner),
                Some('p') => element(Element::Np, scanner),
                _ => Ok(BracketSymbol::Element(Element::N))
            }
        },
        Some('O') => {
            scanner.pop();

            match scanner.peek() {
                Some('g') => element(Element::Og, scanner),
                Some('s') => element(Element::Os, scanner),
                _ => Ok(BracketSymbol::Element(Element::O))
            }
        },
        Some('P') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => element(Element::Pa, scanner),
                Some('b') => element(Element::Pb, scanner),
                Some('d') => element(Element::Pd, scanner),
                Some('m') => element(Element::Pm, scanner),
                Some('o') => element(Element::Po, scanner),
                Some('r') => element(Element::Pr, scanner),
                Some('t') => element(Element::Pt, scanner),
                Some('u') => element(Element::Pu, scanner),
                _ => Ok(BracketSymbol::Element(Element::P))
            }
        },
        Some('R') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => element(Element::Ra, scanner),
                Some('b') => element(Element::Rb, scanner),
                Some('e') => element(Element::Re, scanner),
                Some('f') => element(Element::Rf, scanner),
                Some('g') => element(Element::Rg, scanner),
                Some('h') => element(Element::Rh, scanner),
                Some('n') => element(Element::Rn, scanner),
                Some('u') => element(Element::Ru, scanner),
                _ => Err(missing_character(scanner))
            }
        },
        Some('S') => {
            scanner.pop();

            match scanner.peek() {
                Some('b') => element(Element::Sb, scanner),
                Some('c') => element(Element::Sc, scanner),
                Some('e') => element(Element::Se, scanner),
                Some('g') => element(Element::Sg, scanner),
                Some('i') => element(Element::Si, scanner),
                Some('m') => element(Element::Sm, scanner),
                Some('n') => element(Element::Sn, scanner),
                Some('r') => element(Element::Sr, scanner),
                _ => Ok(BracketSymbol::Element(Element::S))
            }
        },
        Some('T') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => element(Element::Ta, scanner),
                Some('b') => element(Element::Tb, scanner),
                Some('c') => element(Element::Tc, scanner),
                Some('e') => element(Element::Te, scanner),
                Some('h') => element(Element::Th, scanner),
                Some('i') => element(Element::Ti, scanner),
                Some('l') => element(Element::Tl, scanner),
                Some('m') => element(Element::Tm, scanner),
                Some('s') => element(Element::Ts, scanner),
                _ => Err(missing_character(scanner))
            }
        },
        Some('U') => element(Element::U, scanner),
        Some('V') => element(Element::V, scanner),
        Some('W') => element(Element::W, scanner),
        Some('X') => {
            scanner.pop();

            match scanner.peek() {
                Some('e') => element(Element::Xe, scanner),
                _ => Err(missing_character(scanner))
            }
        },
        Some('Y') => {
            scanner.pop();

            match scanner.peek() {
                Some('b') => element(Element::Yb, scanner),
                _ => element(Element::Y, scanner)
            }
        },
        Some('Z') => {
            scanner.pop();

            match scanner.peek() {
                Some('n') => element(Element::Zn, scanner),
                Some('r') => element(Element::Zr, scanner),
                _ => Err(missing_character(scanner))
            }
        },
        _ => Err(missing_character(scanner))
    }
}

fn aromatic(
    aromatic: BracketAromatic, scanner: &mut Scanner
) -> Result<BracketSymbol, Error> {
    scanner.pop();

    Ok(BracketSymbol::Aromatic(aromatic))
}

fn element(
    element: Element, scanner: &mut Scanner
) -> Result<BracketSymbol, Error> {
    scanner.pop();

    Ok(BracketSymbol::Element(element))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn blank() {
        let mut scanner = Scanner::new("");

        assert_eq!(read_symbol(&mut scanner), Err(Error::EndOfLine))
    }

    #[test]
    fn j_eol() {
        let mut scanner = Scanner::new("J");

        assert_eq!(read_symbol(&mut scanner), Err(Error::InvalidCharacter(0)))
    }

    #[test]
    fn lower_a_eol() {
        let mut scanner = Scanner::new("a");

        assert_eq!(read_symbol(&mut scanner), Err(Error::EndOfLine)
        )
    }

    #[test]
    fn lower_ax_eol() {
        let mut scanner = Scanner::new("ax");

        assert_eq!(read_symbol(&mut scanner), Err(Error::InvalidCharacter(1)))
    }

    #[test]
    fn aromatic_eol() {
        let tests = vec![
            ("as", BracketAromatic::As, 2),
            ("b", BracketAromatic::B, 1),
            ("c", BracketAromatic::C, 1),
            ("n", BracketAromatic::N, 1),
            ("o", BracketAromatic::O, 1),
            ("p", BracketAromatic::P, 1),
            ("s", BracketAromatic::S, 1),
            ("se", BracketAromatic::Se, 2)
        ];

        for (input, aromatic, cursor) in tests.into_iter() {
            let mut scanner = Scanner::new(input);

            assert_eq!(read_symbol(&mut scanner), Ok(BracketSymbol::Aromatic(aromatic)));
            assert_eq!(scanner.cursor(), cursor)
        }
    }

    #[test]
    fn upper_a_eol() {
        let mut scanner = Scanner::new(&"A");

        assert_eq!(read_symbol(&mut scanner), Err(Error::EndOfLine));
        assert_eq!(scanner.cursor(), 1);
    }
    
    #[test]
    fn a_unknown() {
        let mut scanner = Scanner::new(&"Ax");

        assert_eq!(
            read_symbol(&mut scanner), Err(Error::InvalidCharacter(1))
        );
        assert_eq!(scanner.cursor(), 1);
    }

    #[test]
    fn element_eol() {
        let tests = vec![
            ("Ac", Element::Ac, 2),
            ("Ag", Element::Ag, 2),
            ("Al", Element::Al, 2),
            ("Am", Element::Am, 2),
            ("Ar", Element::Ar, 2),
            ("As", Element::As, 2),
            ("At", Element::At, 2),
            ("Au", Element::Au, 2),
            ("B", Element::B, 1),
            ("Ba", Element::Ba, 2),
            ("Be", Element::Be, 2),
            ("Bh", Element::Bh, 2),
            ("Bi", Element::Bi, 2),
            ("Bk", Element::Bk, 2),
            ("Br", Element::Br, 2),
            ("C", Element::C, 1),
            ("Ca", Element::Ca, 2),
            ("Cd", Element::Cd, 2),
            ("Ce", Element::Ce, 2),
            ("Cf", Element::Cf, 2),
            ("Cl", Element::Cl, 2),
            ("Cm", Element::Cm, 2),
            ("Co", Element::Co, 2),
            ("Cs", Element::Cs, 2),
            ("Cu", Element::Cu, 2),
            ("Db", Element::Db, 2),
            ("Ds", Element::Ds, 2),
            ("Dy", Element::Dy, 2),
            ("Er", Element::Er, 2),
            ("Es", Element::Es, 2),
            ("Eu", Element::Eu, 2),
            ("F", Element::F, 1),
            ("Fe", Element::Fe, 2),
            ("Fl", Element::Fl, 2),
            ("Fm", Element::Fm, 2),
            ("Fr", Element::Fr, 2),
            ("Ga", Element::Ga, 2),
            ("Gd", Element::Gd, 2),
            ("Ge", Element::Ge, 2),
            ("H", Element::H, 1),
            ("He", Element::He, 2),
            ("Hf", Element::Hf, 2),
            ("Hg", Element::Hg, 2),
            ("Ho", Element::Ho, 2),
            ("Hs", Element::Hs, 2),
            ("I", Element::I, 1),
            ("In", Element::In, 2),
            ("Ir", Element::Ir, 2),
            ("K", Element::K, 1),
            ("Kr", Element::Kr, 2),
            ("La", Element::La, 2),
            ("Li", Element::Li, 2),
            ("Lr", Element::Lr, 2),
            ("Lu", Element::Lu, 2),
            ("Lv", Element::Lv, 2),
            ("Mc", Element::Mc, 2),
            ("Md", Element::Md, 2),
            ("Mg", Element::Mg, 2),
            ("Mn", Element::Mn, 2),
            ("Mo", Element::Mo, 2),
            ("Mt", Element::Mt, 2),
            ("N", Element::N, 1),
            ("Na", Element::Na, 2),
            ("Nb", Element::Nb, 2),
            ("Nd", Element::Nd, 2),
            ("Ne", Element::Ne, 2),
            ("Nh", Element::Nh, 2),
            ("Ni", Element::Ni, 2),
            ("No", Element::No, 2),
            ("Np", Element::Np, 2),
            ("O", Element::O, 1),
            ("Og", Element::Og, 2),
            ("Os", Element::Os, 2),
            ("P", Element::P, 1),
            ("Pa", Element::Pa, 2),
            ("Pd", Element::Pd, 2),
            ("Pm", Element::Pm, 2),
            ("Po", Element::Po, 2),
            ("Pr", Element::Pr, 2),
            ("Pt", Element::Pt, 2),
            ("Pu", Element::Pu, 2),
            ("Ra", Element::Ra, 2),
            ("Rb", Element::Rb, 2),
            ("Re", Element::Re, 2),
            ("Rf", Element::Rf, 2),
            ("Rg", Element::Rg, 2),
            ("Rh", Element::Rh, 2),
            ("Rn", Element::Rn, 2),
            ("Ru", Element::Ru, 2),
            ("S", Element::S, 1),
            ("Sb", Element::Sb, 2),
            ("Sc", Element::Sc, 2),
            ("Se", Element::Se, 2),
            ("Sg", Element::Sg, 2),
            ("Si", Element::Si, 2),
            ("Sm", Element::Sm, 2),
            ("Sn", Element::Sn, 2),
            ("Ta", Element::Ta, 2),
            ("Tb", Element::Tb, 2),
            ("Tc", Element::Tc, 2),
            ("Te", Element::Te, 2),
            ("Th", Element::Th, 2),
            ("Ti", Element::Ti, 2),
            ("Tl", Element::Tl, 2),
            ("Tm", Element::Tm, 2),
            ("Ts", Element::Ts, 2),
            ("U", Element::U, 1),
            ("V", Element::V, 1),
            ("W", Element::W, 1),
            ("Xe", Element::Xe, 2),
            ("Y", Element::Y, 1),
            ("Yb", Element::Yb, 2),
            ("Zn", Element::Zn, 2),
            ("Zr", Element::Zr, 2),
        ];

        for (input, element, cursor) in tests.into_iter() {
            let mut scanner = Scanner::new(input);

            assert_eq!(read_symbol(&mut scanner), Ok(BracketSymbol::Element(element)));
            assert_eq!(scanner.cursor(), cursor)
        }
    }
}