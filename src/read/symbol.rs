use crate::util::Scanner;
use super::error::Error;
use crate::mol::Atom;
use crate::mol::Element;

pub fn symbol(
    scanner: &mut Scanner, atom: &mut Atom
) -> Result<(), Error> {
    if scanner.done() {
        return Err(Error::EndOfLine);
    }

    let element_aromatic = match scanner.pop() {
        Some('b') => Some((Element::B, true)),
        Some('c') => Some((Element::C, true)),
        Some('n') => Some((Element::N, true)),
        Some('o') => Some((Element::O, true)),
        Some('p') => Some((Element::P, true)),
        Some('s') => Some((Element::S, true)),
        Some('A') => {
            match scanner.peek() {
                Some('c') => pop_and_go(Element::Ac, scanner),
                Some('g') => pop_and_go(Element::Ag, scanner),
                Some('l') => pop_and_go(Element::Al, scanner),
                Some('m') => pop_and_go(Element::Am, scanner),
                Some('r') => pop_and_go(Element::Ar, scanner),
                Some('s') => pop_and_go(Element::As, scanner),
                Some('t') => pop_and_go(Element::At, scanner),
                Some('u') => pop_and_go(Element::Au, scanner),
                None => return Err(Error::EndOfLine),
                _ => None
            }
        },
        Some('B') => {
            match scanner.peek() {
                Some('a') => pop_and_go(Element::Ba, scanner),
                Some('e') => pop_and_go(Element::Be, scanner),
                Some('h') => pop_and_go(Element::Bh, scanner),
                Some('i') => pop_and_go(Element::Bi, scanner),
                Some('k') => pop_and_go(Element::Bk, scanner),
                Some('r') => pop_and_go(Element::Br, scanner),
                _ => Some((Element::B, false))
            }
        },
        Some('C') => {
            match scanner.peek() {
                Some('a') => pop_and_go(Element::Ca, scanner),
                Some('d') => pop_and_go(Element::Cd, scanner),
                Some('e') => pop_and_go(Element::Ce, scanner),
                Some('f') => pop_and_go(Element::Cf, scanner),
                Some('l') => pop_and_go(Element::Cl, scanner),
                Some('m') => pop_and_go(Element::Cm, scanner),
                Some('n') => pop_and_go(Element::Cn, scanner),
                Some('o') => pop_and_go(Element::Co, scanner),
                Some('r') => pop_and_go(Element::Cr, scanner),
                Some('s') => pop_and_go(Element::Cs, scanner),
                Some('u') => pop_and_go(Element::Cu, scanner),
                _ => Some((Element::C, false))
            }
        },
        Some('D') => {
            match scanner.peek() {
                Some('b') => pop_and_go(Element::Db, scanner),
                Some('s') => pop_and_go(Element::Ds, scanner),
                Some('y') => pop_and_go(Element::Dy, scanner),
                None => return Err(Error::EndOfLine),
                _ => None
            }
        },
        Some('E') => {
            match scanner.peek() {
                Some('r') => pop_and_go(Element::Er, scanner),
                Some('s') => pop_and_go(Element::Es, scanner),
                Some('u') => pop_and_go(Element::Eu, scanner),
                None => return Err(Error::EndOfLine),
                _ => None
            }
        },
        Some('F') => {
            match scanner.peek() {
                Some('e') => pop_and_go(Element::Fe, scanner),
                Some('l') => pop_and_go(Element::Fl, scanner),
                Some('m') => pop_and_go(Element::Fm, scanner),
                Some('r') => pop_and_go(Element::Fr, scanner),
                _ => Some((Element::F, false))
            }
        },
        Some('G') => {
            match scanner.peek() {
                Some('a') => pop_and_go(Element::Ga, scanner),
                Some('d') => pop_and_go(Element::Gd, scanner),
                Some('e') => pop_and_go(Element::Ge, scanner),
                None => return Err(Error::EndOfLine),
                _ => None
            }
        },
        Some('H') => {
            match scanner.peek() {
                Some('e') => pop_and_go(Element::He, scanner),
                Some('f') => pop_and_go(Element::Hf, scanner),
                Some('g') => pop_and_go(Element::Hg, scanner),
                Some('o') => pop_and_go(Element::Ho, scanner),
                Some('s') => pop_and_go(Element::Hs, scanner),
                _ => Some((Element::H, false))
            }
        },
        Some('I') => {
            match scanner.peek() {
                Some('n') => pop_and_go(Element::In, scanner),
                Some('r') => pop_and_go(Element::Ir, scanner),
                _ => Some((Element::I, false))
            }
        },
        Some('K') => {
            match scanner.peek() {
                Some('r') => pop_and_go(Element::Kr, scanner),
                _ => Some((Element::K, false))
            }
        },
        Some('L') => {
            match scanner.peek() {
                Some('a') => pop_and_go(Element::La, scanner),
                Some('i') => pop_and_go(Element::Li, scanner),
                Some('r') => pop_and_go(Element::Lr, scanner),
                Some('u') => pop_and_go(Element::Lu, scanner),
                Some('v') => pop_and_go(Element::Lv, scanner),
                None => return Err(Error::EndOfLine),
                _ => None
            }
        },
        Some('M') => {
            match scanner.peek() {
                Some('c') => pop_and_go(Element::Mc, scanner),
                Some('d') => pop_and_go(Element::Md, scanner),
                Some('g') => pop_and_go(Element::Mg, scanner),
                Some('n') => pop_and_go(Element::Mn, scanner),
                Some('o') => pop_and_go(Element::Mo, scanner),
                Some('t') => pop_and_go(Element::Mt, scanner),
                None => return Err(Error::EndOfLine),
                _ => None
            }
        },
        Some('N') => {
            match scanner.peek() {
                Some('a') => pop_and_go(Element::Na, scanner),
                Some('b') => pop_and_go(Element::Nb, scanner),
                Some('d') => pop_and_go(Element::Nd, scanner),
                Some('e') => pop_and_go(Element::Ne, scanner),
                Some('h') => pop_and_go(Element::Nh, scanner),
                Some('i') => pop_and_go(Element::Ni, scanner),
                Some('o') => pop_and_go(Element::No, scanner),
                Some('p') => pop_and_go(Element::Np, scanner),
                _ => Some((Element::N, false))
            }
        },
        Some('O') => {
            match scanner.peek() {
                Some('g') => pop_and_go(Element::Og, scanner),
                Some('s') => pop_and_go(Element::Os, scanner),
                _ => Some((Element::O, false))
            }
        },
        Some('P') => {
            match scanner.peek() {
                Some('a') => pop_and_go(Element::Pa, scanner),
                Some('b') => pop_and_go(Element::Pb, scanner),
                Some('d') => pop_and_go(Element::Pd, scanner),
                Some('m') => pop_and_go(Element::Pm, scanner),
                Some('o') => pop_and_go(Element::Po, scanner),
                Some('r') => pop_and_go(Element::Pr, scanner),
                Some('t') => pop_and_go(Element::Pt, scanner),
                Some('u') => pop_and_go(Element::Pu, scanner),
                _ => Some((Element::P, false))
            }
        },
        Some('R') => {
            match scanner.peek() {
                Some('a') => pop_and_go(Element::Ra, scanner),
                Some('b') => pop_and_go(Element::Rb, scanner),
                Some('e') => pop_and_go(Element::Re, scanner),
                Some('f') => pop_and_go(Element::Rf, scanner),
                Some('g') => pop_and_go(Element::Rg, scanner),
                Some('h') => pop_and_go(Element::Rh, scanner),
                Some('n') => pop_and_go(Element::Rn, scanner),
                Some('u') => pop_and_go(Element::Ru, scanner),
                None => return Err(Error::EndOfLine),
                _ => None
            }
        },
        Some('S') => {
            match scanner.peek() {
                Some('b') => pop_and_go(Element::Sb, scanner),
                Some('c') => pop_and_go(Element::Sc, scanner),
                Some('e') => pop_and_go(Element::Se, scanner),
                Some('g') => pop_and_go(Element::Sg, scanner),
                Some('i') => pop_and_go(Element::Si, scanner),
                Some('m') => pop_and_go(Element::Sm, scanner),
                Some('n') => pop_and_go(Element::Sn, scanner),
                Some('r') => pop_and_go(Element::Sr, scanner),
                _ => Some((Element::S, false))
            }
        },
        Some('T') => {
            match scanner.peek() {
                Some('a') => pop_and_go(Element::Ta, scanner),
                Some('b') => pop_and_go(Element::Tb, scanner),
                Some('c') => pop_and_go(Element::Tc, scanner),
                Some('e') => pop_and_go(Element::Te, scanner),
                Some('h') => pop_and_go(Element::Th, scanner),
                Some('i') => pop_and_go(Element::Ti, scanner),
                Some('l') => pop_and_go(Element::Tl, scanner),
                Some('m') => pop_and_go(Element::Tm, scanner),
                Some('s') => pop_and_go(Element::Ts, scanner),
                None => return Err(Error::EndOfLine),
                _ => None
            }
        },
        Some('U') => Some((Element::U, false)),
        Some('V') => Some((Element::V, false)),
        Some('W') => Some((Element::W, false)),
        Some('X') => {
            match scanner.peek() {
                Some('e') => pop_and_go(Element::Xe, scanner),
                None => return Err(Error::EndOfLine),
                _ => None
            }
        },
        Some('Y') => {
            match scanner.peek() {
                Some('b') => pop_and_go(Element::Yb, scanner),
                _ => Some((Element::Yb, false))
            }
        },
        Some('Z') => {
            match scanner.peek() {
                Some('n') => pop_and_go(Element::Zn, scanner),
                Some('r') => pop_and_go(Element::Zr, scanner),
                None => return Err(Error::EndOfLine),
                _ => None
            }
        },
        _ => return Err(Error::InvalidCharacter(scanner.cursor() - 1))
    };

    match element_aromatic {
        Some((element, aromatic)) => {
            atom.element = element;
            atom.aromatic = aromatic;

            Ok(())
        },
        None => Err(Error::InvalidCharacter(scanner.cursor()))
    }
}

fn pop_and_go(
    element: Element, scanner: &mut Scanner
) -> Option<(Element, bool)> {
    scanner.pop();

    Some((element, false))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn blank() {
        let mut scanner = Scanner::new(&"");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Err(Error::EndOfLine));
    }

    #[test]
    fn j_eol() {
        let mut scanner = Scanner::new(&"J");
        let mut atom = Default::default();

        assert_eq!(symbol(
            &mut scanner, &mut atom), Err(Error::InvalidCharacter(0))
        );
    }

    #[test]
    fn aromatic_boron_eol() {
        let mut scanner = Scanner::new(&"b");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom {
            element: Element::B, aromatic: true, ..Default::default()
        });
        assert_eq!(scanner.cursor(), 1);
    }

    #[test]
    fn carbon_eol() {
        let mut scanner = Scanner::new(&"c");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom {
            element: Element::C, aromatic: true, ..Default::default()
        });
        assert_eq!(scanner.cursor(), 1);
    }

    #[test]
    fn aromatic_boron_unknown() {
        let mut scanner = Scanner::new(&"bx");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom {
            element: Element::B, aromatic: true, ..Default::default()
        });
        assert_eq!(scanner.cursor(), 1);
    }

    #[test]
    fn a_eol() {
        let mut scanner = Scanner::new(&"A");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Err(Error::EndOfLine));
        assert_eq!(scanner.cursor(), 1);
    }
    
    #[test]
    fn a_unknown() {
        let mut scanner = Scanner::new(&"Ax");
        let mut atom = Default::default();

        assert_eq!(
            symbol(&mut scanner, &mut atom), Err(Error::InvalidCharacter(1))
        );
        assert_eq!(scanner.cursor(), 1);
    }

    #[test]
    fn actinium_eol() {
        let mut scanner = Scanner::new(&"Ac");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Ac, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn boron_eol() {
        let mut scanner = Scanner::new(&"B");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::B, ..Default::default() });
        assert_eq!(scanner.cursor(), 1);
    }

    #[test]
    fn boron_unknown() {
        let mut scanner = Scanner::new(&"Bx");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::B, ..Default::default() });
        assert_eq!(scanner.cursor(), 1);
    }

    #[test]
    fn barium_eol() {
        let mut scanner = Scanner::new(&"Ba");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Ba, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn dubnium_eol() {
        let mut scanner = Scanner::new(&"Db");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Db, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn einsteinium_eol() {
        let mut scanner = Scanner::new(&"Es");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Es, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn iron_eol() {
        let mut scanner = Scanner::new(&"Fe");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Fe, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn gadolinium_eol() {
        let mut scanner = Scanner::new(&"Ga");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Ga, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn mercury_eol() {
        let mut scanner = Scanner::new(&"Hg");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Hg, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn indium_eol() {
        let mut scanner = Scanner::new(&"In");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::In, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn krypton_eol() {
        let mut scanner = Scanner::new(&"Kr");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Kr, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn lutetiium_eol() {
        let mut scanner = Scanner::new(&"Lu");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Lu, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn magnesioum_eol() {
        let mut scanner = Scanner::new(&"Mg");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Mg, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn niobium_eol() {
        let mut scanner = Scanner::new(&"Nb");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Nb, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn osmium_eol() {
        let mut scanner = Scanner::new(&"Os");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Os, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn palladium_eol() {
        let mut scanner = Scanner::new(&"Pd");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Pd, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn rutherfordium_eol() {
        let mut scanner = Scanner::new(&"Rf");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Rf, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn seaborgium_eol() {
        let mut scanner = Scanner::new(&"Sg");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Sg, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn throrium_eol() {
        let mut scanner = Scanner::new(&"Tl");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Tl, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn uranium_eol() {
        let mut scanner = Scanner::new(&"U");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::U, ..Default::default() });
        assert_eq!(scanner.cursor(), 1);
    }

    #[test]
    fn vanadium_eol() {
        let mut scanner = Scanner::new(&"V");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::V, ..Default::default() });
        assert_eq!(scanner.cursor(), 1);
    }

    #[test]
    fn tungsten_eol() {
        let mut scanner = Scanner::new(&"W");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::W, ..Default::default() });
        assert_eq!(scanner.cursor(), 1);
    }

    #[test]
    fn xenon_eol() {
        let mut scanner = Scanner::new(&"Xe");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Xe, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }
    
    #[test]
    fn ytterbium_eol() {
        let mut scanner = Scanner::new(&"Yb");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Yb, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }

    #[test]
    fn zirconium_eol() {
        let mut scanner = Scanner::new(&"Zr");
        let mut atom = Default::default();

        assert_eq!(symbol(&mut scanner, &mut atom), Ok(()));
        assert_eq!(atom, Atom { element: Element::Zr, ..Default::default() });
        assert_eq!(scanner.cursor(), 2);
    }
}