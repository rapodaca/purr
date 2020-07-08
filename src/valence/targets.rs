use crate::mol::{ Element, Nub };

/// Returns valence target for Atom. Uses the valence model presented in
/// [Hydrogen Suppression in SMILES](https://depth-first.com/articles/2020/06/08/hydrogen-suppression-in-smiles/)
pub fn targets(nub: &Nub) -> Option<Vec<u8>> {
    match isoform(nub) {
        Some(Isoform::Boron) => Some(vec![ 3 ]),
        Some(Isoform::Carbon) => Some(vec![ 4 ]),
        Some(Isoform::Nitrogen) => Some(vec![ 3, 5 ]),
        Some(Isoform::Oxygen) => Some(vec![ 2 ]),
        Some(Isoform::Phosphorous) => Some(vec![ 3, 5 ]),
        Some(Isoform::Sulfur) => Some(vec![ 2, 4, 6 ]),
        Some(Isoform::Halogen) => Some(vec![ 1 ]),
        None => None
    }
}

fn isoform(nub: &Nub) -> Option<Isoform> {
    match nub.element {
        Element::B => {
            match nub.charge {
                None => Some(Isoform::Boron),
                Some(-4) => Some(Isoform::Halogen),
                Some(-3) => Some(Isoform::Oxygen),
                Some(-2) => Some(Isoform::Nitrogen),
                Some(-1) => Some(Isoform::Carbon),
                Some(0) => Some(Isoform::Boron),
                _ => None
            }
        },
        Element::C => {
            match nub.charge {
                None => Some(Isoform::Carbon),
                Some(-3) => Some(Isoform::Halogen),
                Some(-2) => Some(Isoform::Oxygen),
                Some(-1) => Some(Isoform::Nitrogen),
                Some(0) => Some(Isoform::Carbon),
                Some(1) => Some(Isoform::Boron),
                _ => None
            }
        },
        Element::N => {
            match nub.charge {
                None => Some(Isoform::Nitrogen),
                Some(-2) => Some(Isoform::Halogen),
                Some(-1) => Some(Isoform::Oxygen),
                Some(0) => Some(Isoform::Nitrogen),
                Some(1) => Some(Isoform::Carbon),
                Some(2) => Some(Isoform::Boron),
                _ => None
            }
        },
        Element::O => {
            match nub.charge {
                None => Some(Isoform::Oxygen),
                Some(-1) => Some(Isoform::Halogen),
                Some(0) => Some(Isoform::Oxygen),
                Some(1) => Some(Isoform::Nitrogen),
                Some(2) => Some(Isoform::Carbon),
                Some(3) => Some(Isoform::Boron),
                _ => None
            }
        },
        Element::F | Element::Cl | Element::Br | Element::I | Element::As |
        Element::Ts => {
            match nub.charge {
                None => Some(Isoform::Halogen),
                Some(0) => Some(Isoform::Halogen),
                Some(1) => Some(Isoform::Oxygen),
                Some(2) => Some(Isoform::Nitrogen),
                Some(3) => Some(Isoform::Carbon),
                Some(4) => Some(Isoform::Boron),
                _ => None
            }
        },
        Element::P => {
            match nub.charge {
                None => Some(Isoform::Phosphorous),
                Some(-2) => Some(Isoform::Halogen),
                Some(-1) => Some(Isoform::Sulfur),
                Some(0) => Some(Isoform::Phosphorous),
                _ => None
            }
        },
        Element::S => {
            match nub.charge {
                None => Some(Isoform::Sulfur),
                Some(-1) => Some(Isoform::Halogen),
                Some(0) => Some(Isoform::Sulfur),
                Some(1) => Some(Isoform::Phosphorous),
                _ => None
            }
        }
        _ => None
    }
}

enum Isoform {
    Boron,
    Carbon,
    Nitrogen,
    Oxygen,
    Phosphorous,
    Sulfur,
    Halogen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boron() {
        let nub = Nub { element: Element::B, ..Default::default() };

        assert_eq!(targets(&nub), Some(vec![ 3 ]));
    }

    #[test]
    fn boron_minus_five() {
        let nub = Nub {
            element: Element::B, charge: Some(-5), ..Default::default()
        };

        assert_eq!(targets(&nub), None);
    }

    #[test]
    fn boron_minus_four() {
        let nub = Nub {
            element: Element::B, charge: Some(-4), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 1 ]));
    }

    #[test]
    fn boron_minus_three() {
        let nub = Nub {
            element: Element::B, charge: Some(-3), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 2 ]));
    }

    #[test]
    fn boron_minus_two() {
        let nub = Nub {
            element: Element::B, charge: Some(-2), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 3, 5 ]));
    }

    #[test]
    fn boron_minus_one() {
        let nub = Nub {
            element: Element::B, charge: Some(-1), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 4 ]));
    }

    #[test]
    fn boron_zero() {
        let nub = Nub {
            element: Element::B, charge: Some(0), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 3 ]));
    }

    #[test]
    fn boron_plus_one() {
        let nub = Nub {
            element: Element::B, charge: Some(1), ..Default::default()
        };

        assert_eq!(targets(&nub), None);
    }

    #[test]
    fn carbon() {
        let nub = Nub { element: Element::C, ..Default::default() };

        assert_eq!(targets(&nub), Some(vec![ 4 ]));
    }

    #[test]
    fn carbon_zero() {
        let nub = Nub {
            element: Element::C, charge: Some(0), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 4 ]));
    }

    #[test]
    fn carbon_plus() {
        let nub = Nub {
            element: Element::C, charge: Some(1), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 3 ]));
    }

    #[test]
    fn carbon_plus_two() {
        let nub = Nub {
            element: Element::C, charge: Some(2), ..Default::default()
        };

        assert_eq!(targets(&nub), None);
    }

    #[test]
    fn carbon_minus() {
        let nub = Nub {
            element: Element::C, charge: Some(-1), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 3, 5 ]));
    }

    #[test]
    fn carbon_minus_two() {
        let nub = Nub {
            element: Element::C, charge: Some(-2), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 2 ]));
    }

    #[test]
    fn carbon_minus_three() {
        let nub = Nub {
            element: Element::C, charge: Some(-3), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 1 ]));
    }

    #[test]
    fn carbon_minus_four() {
        let nub = Nub {
            element: Element::C, charge: Some(-4), ..Default::default()
        };

        assert_eq!(targets(&nub), None);
    }

    #[test]
    fn nitrogen() {
        let nub = Nub { element: Element::N, ..Default::default() };

        assert_eq!(targets(&nub), Some(vec![ 3, 5 ]));
    }

    #[test]
    fn nitrogen_minus_three() {
        let nub = Nub {
            element: Element::N, charge: Some(-3), ..Default::default()
        };

        assert_eq!(targets(&nub), None);
    }

    #[test]
    fn nitrogen_minus_two() {
        let nub = Nub {
            element: Element::N, charge: Some(-2), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 1 ]));
    }

    #[test]
    fn nitrogen_minus_one() {
        let nub = Nub {
            element: Element::N, charge: Some(-1), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 2 ]));
    }

    #[test]
    fn nitrogen_zero() {
        let nub = Nub {
            element: Element::N, charge: Some(0), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 3, 5 ]));
    }

    #[test]
    fn nitrogen_plus_one() {
        let nub = Nub {
            element: Element::N, charge: Some(1), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 4 ]));
    }

    #[test]
    fn nitrogen_plus_two() {
        let nub = Nub {
            element: Element::N, charge: Some(2), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 3 ]));
    }

    #[test]
    fn nitrogen_plus_three() {
        let nub = Nub {
            element: Element::N, charge: Some(3), ..Default::default()
        };

        assert_eq!(targets(&nub), None);
    }

    #[test]
    fn oxygen() {
        let nub = Nub { element: Element::O, ..Default::default() };

        assert_eq!(targets(&nub), Some(vec![ 2 ]));
    }

    #[test]
    fn oxygen_minus_two() {
        let nub = Nub {
            element: Element::O, charge: Some(-2), ..Default::default()
        };

        assert_eq!(targets(&nub), None);
    }

    #[test]
    fn oxygen_minus_one() {
        let nub = Nub {
            element: Element::O, charge: Some(-1), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 1 ]));
    }

    #[test]
    fn oxygen_zero() {
        let nub = Nub {
            element: Element::O, charge: Some(0), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 2 ]));
    }

    #[test]
    fn oxygen_plus_one() {
        let nub = Nub {
            element: Element::O, charge: Some(1), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 3, 5 ]));
    }

    #[test]
    fn oxygen_plus_two() {
        let nub = Nub {
            element: Element::O, charge: Some(2), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 4 ]));
    }

    #[test]
    fn oxygen_plus_three() {
        let nub = Nub {
            element: Element::O, charge: Some(3), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 3 ]));
    }

    #[test]
    fn fluorine() {
        let nub = Nub { element: Element::F, ..Default::default() };

        assert_eq!(targets(&nub), Some(vec![ 1 ]));
    }

    #[test]
    fn fluorine_minus_one() {
        let nub = Nub {
            element: Element::F, charge: Some(-1), ..Default::default()
        };

        assert_eq!(targets(&nub), None);
    }

    #[test]
    fn fluorine_zero() {
        let nub = Nub {
            element: Element::F, charge: Some(0), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 1 ]));
    }

    #[test]
    fn fluorine_plus_one() {
        let nub = Nub {
            element: Element::F, charge: Some(1), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 2 ]));
    }

    #[test]
    fn fluorine_plus_two() {
        let nub = Nub {
            element: Element::F, charge: Some(2), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 3, 5 ]));
    }

    #[test]
    fn fluorine_plus_three() {
        let nub = Nub {
            element: Element::F, charge: Some(3), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 4 ]));
    }

    #[test]
    fn fluorine_plus_four() {
        let nub = Nub {
            element: Element::F, charge: Some(4), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 3 ]));
    }

    #[test]
    fn chlorine() {
        let nub = Nub { element: Element::Cl, ..Default::default() };

        assert_eq!(targets(&nub), Some(vec![ 1 ]));
    }

    #[test]
    fn bromine() {
        let nub = Nub { element: Element::Br, ..Default::default() };

        assert_eq!(targets(&nub), Some(vec![ 1 ]));
    }

    #[test]
    fn iodine() {
        let nub = Nub { element: Element::I, ..Default::default() };

        assert_eq!(targets(&nub), Some(vec![ 1 ]));
    }

    #[test]
    fn astatine() {
        let nub = Nub { element: Element::As, ..Default::default() };

        assert_eq!(targets(&nub), Some(vec![ 1 ]));
    }

    #[test]
    fn tennesine() {
        let nub = Nub { element: Element::Ts, ..Default::default() };

        assert_eq!(targets(&nub), Some(vec![ 1 ]));
    }

    #[test]
    fn phosphorous() {
        let nub = Nub { element: Element::P, ..Default::default() };

        assert_eq!(targets(&nub), Some(vec![ 3, 5 ]));
    }

    #[test]
    fn phosphorous_minus_three() {
        let nub = Nub {
            element: Element::P, charge: Some(-3), ..Default::default()
        };

        assert_eq!(targets(&nub), None);
    }

    #[test]
    fn phosphorous_minus_two() {
        let nub = Nub {
            element: Element::P, charge: Some(-2), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 1 ]));
    }

    #[test]
    fn phosphorous_minus_one() {
        let nub = Nub {
            element: Element::P, charge: Some(-1), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 2, 4, 6 ]));
    }

    #[test]
    fn phosphorous_zero() {
        let nub = Nub {
            element: Element::P, charge: Some(0), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 3, 5]));
    }

    #[test]
    fn phosphorous_plus_one() {
        let nub = Nub {
            element: Element::P, charge: Some(1), ..Default::default()
        };

        assert_eq!(targets(&nub), None);
    }

    #[test]
    fn sulfur() {
        let nub = Nub { element: Element::S, ..Default::default() };

        assert_eq!(targets(&nub), Some(vec![ 2, 4, 6 ]));
    }

    #[test]
    fn sulfur_minus_two() {
        let nub = Nub {
            element: Element::S, charge: Some(-2), ..Default::default()
        };

        assert_eq!(targets(&nub), None);
    }

    #[test]
    fn sulfur_minus_one() {
        let nub = Nub {
            element: Element::S, charge: Some(-1), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 1 ]));
    }

    #[test]
    fn sulfur_zero() {
        let nub = Nub {
            element: Element::S, charge: Some(0), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 2, 4, 6 ]));
    }

    #[test]
    fn sulfur_plus_one() {
        let nub = Nub {
            element: Element::S, charge: Some(1), ..Default::default()
        };

        assert_eq!(targets(&nub), Some(vec![ 3, 5 ]));
    }

    #[test]
    fn sulfur_plus_two() {
        let nub = Nub {
            element: Element::S, charge: Some(2), ..Default::default()
        };

        assert_eq!(targets(&nub), None);
    }
}