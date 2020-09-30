use crate::mol::{ Atom };
use super::{ targets, valence, Error };

/// Returns the difference between the next-nearest target valence and
/// the bond order sum. Returns None if valence targets are not defined.
/// Includes hcount, but disregards aromatic flag.
/// Returns error if target valences are available but none match.
pub fn hypovalence(atom: &Atom) -> Result<Option<u8>, Error> {
    match targets(&atom.nub) {
        Some(targets) => {
            let sum = valence(&atom.bonds);

            for target in targets {
                if target >= sum {
                    match atom.nub.hcount {
                        Some(hcount) => return Ok(Some(target - sum - hcount)),
                        None => return Ok(Some(target - sum))
                    }
                }
            }
            
            Ok(Some(0))
        },
        None => Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mol::{ Bond, Style, Element, Nub };

    #[test]
    fn boron() {
        let atom = Atom {
            nub: Nub { element: Element::B, ..Default::default() },
            bonds: vec![ ]
        };

        assert_eq!(hypovalence(&atom), Ok(Some(3)));
    }

    #[test]
    fn boron_none() {
        let atom = Atom {
            nub: Nub { element: Element::B, ..Default::default() },
            bonds: vec![
                Bond { tid: 1, style: None }
            ]
        };

        assert_eq!(hypovalence(&atom), Ok(Some(2)));
    }

    #[test]
    fn boron_single_none_none() {
        let atom = Atom {
            nub: Nub { element: Element::B, ..Default::default() },
            bonds: vec![
                Bond { tid: 1, style: Some(Style::Single) },
                Bond { tid: 2, style: None },
                Bond { tid: 3, style: None }
            ]
        };

        assert_eq!(hypovalence(&atom), Ok(Some(0)));
    }

    #[test]
    fn chlorine_none_none() {
        let atom = Atom {
            nub: Nub { element: Element::Cl, ..Default::default() },
            bonds: vec![
                Bond { tid: 1, style: None },
                Bond { tid: 2, style: None }
            ]
        };

        assert_eq!(hypovalence(&atom), Ok(Some(0)));
    }

    #[test]
    fn carbon_one_hydrogen() {
        let atom = Atom {
            nub: Nub {
                element: Element::C, hcount: Some(1), ..Default::default()
            },
            bonds: vec![ ]
        };

        assert_eq!(hypovalence(&atom), Ok(Some(3)));
    }

    #[test]
    fn carbon_single_single() {
        let atom = Atom {
            nub: Nub {
                element: Element::C, ..Default::default()
            },
            bonds: vec![
                Bond { tid: 1, style: Some(Style::Single) },
                Bond { tid: 2, style: Some(Style::Single) },
            ]
        };
        
        assert_eq!(hypovalence(&atom), Ok(Some(2)));
    }

    #[test]
    fn carbon_double_up() {
        let atom = Atom {
            nub: Nub {
                element: Element::C, ..Default::default()
            },
            bonds: vec![
                Bond { tid: 1, style: Some(Style::Double) },
                Bond { tid: 2, style: Some(Style::Up) },
            ]
        };

        assert_eq!(hypovalence(&atom), Ok(Some(1)));
    }

    #[test]
    fn carbon_aromatic_aromatic_single() {
        let atom = Atom {
            nub: Nub {
                element: Element::C, ..Default::default()
            },
            bonds: vec![
                Bond { tid: 1, style: Some(Style::Aromatic) },
                Bond { tid: 2, style: Some(Style::Aromatic) },
                Bond { tid: 3, style: Some(Style::Single) },
            ]
        };

        assert_eq!(hypovalence(&atom), Ok(Some(1)));
    }

    #[test]
    fn nitrogen_single() {
        let atom = Atom {
            nub: Nub {
                element: Element::N, ..Default::default()
            },
            bonds: vec![
                Bond { tid: 1, style: Some(Style::Single) },
            ]
        };

        assert_eq!(hypovalence(&atom), Ok(Some(2)));
    }

    #[test]
    fn nitrogen_double_none_none() {
        let atom = Atom {
            nub: Nub {
                element: Element::N, ..Default::default()
            },
            bonds: vec![
                Bond { tid: 1, style: Some(Style::Double) },
                Bond { tid: 2, style: None },
                Bond { tid: 3, style: None },
            ]
        };

        assert_eq!(hypovalence(&atom), Ok(Some(1)));
    }

    #[test]
    fn titanium_none_none() {
        let atom = Atom {
            nub: Nub {
                element: Element::Ti, ..Default::default()
            },
            bonds: vec![
                Bond { tid: 2, style: None },
                Bond { tid: 3, style: None },
            ]
        };

        assert_eq!(hypovalence(&atom), Ok(None));
    }
}