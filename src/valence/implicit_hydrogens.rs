use crate::mol::{ Atom, Style };
use super::{ hypovalence, Error };

/// Returns the implicit hydrogen count for the atom, or None if the
/// atom isn't a member of the valence model.
pub fn implicit_hydrogens(atom: &Atom) -> Result<Option<u8>, Error> {
    match atom.nub.hcount {
        Some(_) => Ok(None),
        None => {
            match hypovalence(atom)? {
                Some(delta) => {
                    if delta > 0 && aromatic(atom) {
                        Ok(Some(delta - 1))
                    } else {
                        Ok(Some(delta))
                    }
                },
                None => Ok(None)
            }
        }
    }
}

fn aromatic(atom: &Atom) -> bool {
    if atom.nub.aromatic {
        return true;
    }

    atom.bonds.iter().any(|bond| {
        match bond.style {
            Some(Style::Aromatic) => true,
            _ => false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mol::Element;
    use crate::mol::Nub;
    use crate::mol::Bond;

    #[test]
    fn default() {
        let atom = Atom {
            nub: Default::default(), bonds: vec![ ]
        };

        assert_eq!(implicit_hydrogens(&atom), Ok(Some(4)));
    }

    #[test]
    fn zero_h() {
        let atom = Atom {
            nub: Nub { hcount: Some(0), ..Default::default() }, bonds: vec![ ]
        };

        assert_eq!(implicit_hydrogens(&atom), Ok(None));
    }

    #[test]
    fn two_h() {
        let atom = Atom {
            nub: Nub { hcount: Some(2), ..Default::default() }, bonds: vec![ ]
        };

        assert_eq!(implicit_hydrogens(&atom), Ok(None));
    }

    #[test]
    fn aromatic_flag() {
        let atom = Atom {
            nub: Nub { aromatic: true, ..Default::default() }, bonds: vec![ ]
        };

        assert_eq!(implicit_hydrogens(&atom), Ok(Some(3)));
    }

    #[test]
    fn aromatic_bond() {
        let atom = Atom {
            nub: Nub { ..Default::default() }, bonds: vec![
                Bond { tid: 1, style: Some(Style::Aromatic) }
            ]
        };

        assert_eq!(implicit_hydrogens(&atom), Ok(Some(2)));
    }

    #[test]
    fn nonmember_element() {
        let atom = Atom {
            nub: Nub {
                element: Element::Sn, ..Default::default()
            }, bonds: vec![
                Bond { tid: 1, style: Some(Style::Aromatic) }
            ]
        };

        assert_eq!(implicit_hydrogens(&atom), Ok(None));
    }

    // see: https://www.slideshare.net/NextMoveSoftware/a-de-facto-standard-or-a-freeforall (slide 16)
    #[test]
    fn overconnected() {
        let atom = Atom {
            nub: Nub {
                element: Element::Cl, ..Default::default()
            }, bonds: vec![
                Bond { tid: 1, style: None },
                Bond { tid: 2, style: None }
            ]
        };

        assert_eq!(implicit_hydrogens(&atom), Ok(Some(0)));
    }
}