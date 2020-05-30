use crate::mol::Element;
use crate::mol::Bond;
use super::valence::valence;

pub fn valence_gap(element: &Element, bonds: &Vec<Bond>) -> Option<u8> {
    let targets = match element {
        Element::B => Some(vec![ 3 ]),
        Element::C => Some(vec![ 4 ]),
        Element::N => Some(vec![ 3, 5 ]),
        Element::O => Some(vec![ 2 ]),
        Element::P => Some(vec![ 3, 5 ]),
        Element::S => Some(vec![ 2, 4, 6 ]),
        Element::F => Some(vec![ 1 ]),
        Element::Cl => Some(vec![ 1 ]),
        Element::Br => Some(vec![ 1 ]),
        Element::I => Some(vec![ 1 ]),
        _ => None
    };

    match targets {
        Some(targets) => {
            let valence = valence(bonds);

            for target in targets {
                if target >= valence {
                    return Some(target - valence);
                }
            }

            None
        },
        None => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mol::Style;

    #[test]
    fn boron() {
        let result = valence_gap(&Element::B, &vec![ ]);

        assert_eq!(result, Some(3));
    }

    #[test]
    fn boron_none() {
        let result = valence_gap(&Element::B, &vec![
            Bond { tid: 0, style: None }
        ]);

        assert_eq!(result, Some(2));
    }

    #[test]
    fn boron_single_none_none() {
        let result = valence_gap(&Element::B, &vec![
            Bond { tid: 0, style: Some(Style::Single) },
            Bond { tid: 1, style: None },
            Bond { tid: 2, style: None }
        ]);

        assert_eq!(result, Some(0));
    }

    #[test]
    fn carbon_single_single() {
        let result = valence_gap(&Element::C, &vec![
            Bond { tid: 0, style: Some(Style::Single) },
            Bond { tid: 1, style: Some(Style::Single) }
        ]);

        assert_eq!(result, Some(2));
    }

    #[test]
    fn carbon_double_up() {
        let result = valence_gap(&Element::C, &vec![
            Bond { tid: 0, style: Some(Style::Double) },
            Bond { tid: 1, style: Some(Style::Up) }
        ]);

        assert_eq!(result, Some(1));
    }

    #[test]
    fn carbon_atomatic_aromatic_single() {
        let result = valence_gap(&Element::C, &vec![
            Bond { tid: 0, style: Some(Style::Aromatic) },
            Bond { tid: 1, style: Some(Style::Aromatic) },
            Bond { tid: 2, style: Some(Style::Single) }
        ]);

        assert_eq!(result, Some(1));
    }

    #[test]
    fn nitrogen_single() {
        let result = valence_gap(&Element::N, &vec![
            Bond { tid: 0, style: Some(Style::Single) }
        ]);

        assert_eq!(result, Some(2));
    }

    #[test]
    fn nitrogen_double_none_none() {
        let result = valence_gap(&Element::N, &vec![
            Bond { tid: 0, style: Some(Style::Double) },
            Bond { tid: 1, style: None },
            Bond { tid: 2, style: None }
        ]);

        assert_eq!(result, Some(1));
    }

    #[test]
    fn oxygen_none() {
        let result = valence_gap(&Element::O, &vec![
            Bond { tid: 0, style: None }
        ]);

        assert_eq!(result, Some(1));
    }

    #[test]
    fn phosphorous_none() {
        let result = valence_gap(&Element::P, &vec![
            Bond { tid: 0, style: None }
        ]);

        assert_eq!(result, Some(2));
    }

    #[test]
    fn phosphorous_double_none_none() {
        let result = valence_gap(&Element::P, &vec![
            Bond { tid: 0, style: Some(Style::Double) },
            Bond { tid: 1, style: None },
            Bond { tid: 2, style: None }
        ]);

        assert_eq!(result, Some(1));
    }

    #[test]
    fn sulfur_none() {
        let result = valence_gap(&Element::S, &vec![
            Bond { tid: 0, style: None }
        ]);

        assert_eq!(result, Some(1));
    }

    #[test]
    fn sulfur_none_none_none() {
        let result = valence_gap(&Element::S, &vec![
            Bond { tid: 0, style: None },
            Bond { tid: 1, style: None },
            Bond { tid: 2, style: None }
        ]);

        assert_eq!(result, Some(1));
    }

    #[test]
    fn sulfur_double_double_none() {
        let result = valence_gap(&Element::S, &vec![
            Bond { tid: 0, style: Some(Style::Double) },
            Bond { tid: 1, style: Some(Style::Double) },
            Bond { tid: 2, style: None }
        ]);

        assert_eq!(result, Some(1));
    }

    #[test]
    fn fluorine() {
        let result = valence_gap(&Element::F, &vec![ ]);

        assert_eq!(result, Some(1));
    }

    #[test]
    fn fluorine_none() {
        let result = valence_gap(&Element::F, &vec![
            Bond { tid: 0, style: None }
        ]);

        assert_eq!(result, Some(0));
    }

    #[test]
    fn chlorine() {
        let result = valence_gap(&Element::Cl, &vec![ ]);

        assert_eq!(result, Some(1));
    }

    #[test]
    fn bromine() {
        let result = valence_gap(&Element::Br, &vec![ ]);

        assert_eq!(result, Some(1));
    }

    #[test]
    fn iodine() {
        let result = valence_gap(&Element::I, &vec![ ]);

        assert_eq!(result, Some(1));
    }
}