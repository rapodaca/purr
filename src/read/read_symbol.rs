use crate::feature::{ Element, BracketSymbol, BracketAromatic };
use super::{ scanner::Scanner, Error, missing_character::missing_character };

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
                _ => Ok(BracketSymbol::Aromatic(BracketAromatic::S))
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
                _ => Ok(BracketSymbol::Element(Element::Y))
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
mod follower {
    use super::*;

    #[test]
    fn blank() {
        let mut scanner = Scanner::new("");

        assert_eq!(read_symbol(&mut scanner), Err(Error::EndOfLine))
    }

    #[test]
    fn j_eol() {
        let mut scanner = Scanner::new("J");

        assert_eq!(read_symbol(&mut scanner), Err(Error::Character(0)))
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

        assert_eq!(read_symbol(&mut scanner), Err(Error::Character(1)))
    }

    #[test]
    fn aromatic_eol() {
        let tests = vec![
            ("asx", BracketAromatic::As, 2),
            ("bx", BracketAromatic::B, 1),
            ("cx", BracketAromatic::C, 1),
            ("nx", BracketAromatic::N, 1),
            ("ox", BracketAromatic::O, 1),
            ("px", BracketAromatic::P, 1),
            ("sx", BracketAromatic::S, 1),
            ("sex", BracketAromatic::Se, 2)
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
            read_symbol(&mut scanner), Err(Error::Character(1))
        );
        assert_eq!(scanner.cursor(), 1);
    }

    #[test]
    fn element_eol() {
        let tests = vec![
            ("Acx", Element::Ac, 2),
            ("Agx", Element::Ag, 2),
            ("Alx", Element::Al, 2),
            ("Amx", Element::Am, 2),
            ("Arx", Element::Ar, 2),
            ("Asx", Element::As, 2),
            ("Atx", Element::At, 2),
            ("Aux", Element::Au, 2),
            ("Bx", Element::B, 1),
            ("Bax", Element::Ba, 2),
            ("Bex", Element::Be, 2),
            ("Bhx", Element::Bh, 2),
            ("Bix", Element::Bi, 2),
            ("Bkx", Element::Bk, 2),
            ("Brx", Element::Br, 2),
            ("Cx", Element::C, 1),
            ("Cax", Element::Ca, 2),
            ("Cdx", Element::Cd, 2),
            ("Cex", Element::Ce, 2),
            ("Cfx", Element::Cf, 2),
            ("Clx", Element::Cl, 2),
            ("Cmx", Element::Cm, 2),
            ("Cox", Element::Co, 2),
            ("Csx", Element::Cs, 2),
            ("Cux", Element::Cu, 2),
            ("Dbx", Element::Db, 2),
            ("Dsx", Element::Ds, 2),
            ("Dyx", Element::Dy, 2),
            ("Erx", Element::Er, 2),
            ("Esx", Element::Es, 2),
            ("Eux", Element::Eu, 2),
            ("Fx", Element::F, 1),
            ("Fex", Element::Fe, 2),
            ("Flx", Element::Fl, 2),
            ("Fmx", Element::Fm, 2),
            ("Frx", Element::Fr, 2),
            ("Gax", Element::Ga, 2),
            ("Gdx", Element::Gd, 2),
            ("Gex", Element::Ge, 2),
            ("Hx", Element::H, 1),
            ("Hex", Element::He, 2),
            ("Hfx", Element::Hf, 2),
            ("Hgx", Element::Hg, 2),
            ("Hox", Element::Ho, 2),
            ("Hsx", Element::Hs, 2),
            ("Ix", Element::I, 1),
            ("Inx", Element::In, 2),
            ("Irx", Element::Ir, 2),
            ("Kx", Element::K, 1),
            ("Krx", Element::Kr, 2),
            ("Lax", Element::La, 2),
            ("Lix", Element::Li, 2),
            ("Lrx", Element::Lr, 2),
            ("Lux", Element::Lu, 2),
            ("Lvx", Element::Lv, 2),
            ("Mcx", Element::Mc, 2),
            ("Mdx", Element::Md, 2),
            ("Mgx", Element::Mg, 2),
            ("Mnx", Element::Mn, 2),
            ("Mox", Element::Mo, 2),
            ("Mtx", Element::Mt, 2),
            ("Nx", Element::N, 1),
            ("Nax", Element::Na, 2),
            ("Nbx", Element::Nb, 2),
            ("Ndx", Element::Nd, 2),
            ("Nex", Element::Ne, 2),
            ("Nhx", Element::Nh, 2),
            ("Nix", Element::Ni, 2),
            ("Nox", Element::No, 2),
            ("Npx", Element::Np, 2),
            ("Ox", Element::O, 1),
            ("Ogx", Element::Og, 2),
            ("Osx", Element::Os, 2),
            ("Px", Element::P, 1),
            ("Pax", Element::Pa, 2),
            ("Pdx", Element::Pd, 2),
            ("Pmx", Element::Pm, 2),
            ("Pox", Element::Po, 2),
            ("Prx", Element::Pr, 2),
            ("Ptx", Element::Pt, 2),
            ("Pux", Element::Pu, 2),
            ("Rax", Element::Ra, 2),
            ("Rbx", Element::Rb, 2),
            ("Rex", Element::Re, 2),
            ("Rfx", Element::Rf, 2),
            ("Rgx", Element::Rg, 2),
            ("Rhx", Element::Rh, 2),
            ("Rnx", Element::Rn, 2),
            ("Rux", Element::Ru, 2),
            ("Sx", Element::S, 1),
            ("Sbx", Element::Sb, 2),
            ("Scx", Element::Sc, 2),
            ("Sex", Element::Se, 2),
            ("Sgx", Element::Sg, 2),
            ("Six", Element::Si, 2),
            ("Smx", Element::Sm, 2),
            ("Snx", Element::Sn, 2),
            ("Tax", Element::Ta, 2),
            ("Tbx", Element::Tb, 2),
            ("Tcx", Element::Tc, 2),
            ("Tex", Element::Te, 2),
            ("Thx", Element::Th, 2),
            ("Tix", Element::Ti, 2),
            ("Tlx", Element::Tl, 2),
            ("Tmx", Element::Tm, 2),
            ("Tsx", Element::Ts, 2),
            ("Ux", Element::U, 1),
            ("Vx", Element::V, 1),
            ("Wx", Element::W, 1),
            ("Xex", Element::Xe, 2),
            ("Yx", Element::Y, 1),
            ("Ybx", Element::Yb, 2),
            ("Znx", Element::Zn, 2),
            ("Zrx", Element::Zr, 2),
        ];

        for (input, element, cursor) in tests.into_iter() {
            let mut scanner = Scanner::new(input);
            let symbol = read_symbol(&mut scanner);

            assert_eq!(symbol, Ok(BracketSymbol::Element(element)));
            assert_eq!(scanner.cursor(), cursor)
        }
    }
}