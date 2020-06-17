use crate::mol::{ Bond, Style };

/// Returns the sum of bond orders using this model:
/// - single: 1
/// - double: 2
/// - triple: 3
/// - quadruple: 4
/// - aromatic: 1
/// - up: 1
/// - down: 1
/// - elided: 1
pub fn valence(bonds: &Vec<Bond>) -> u8 {
    bonds.iter().fold(0, |total, bond| {
        match bond.style {
            Some(Style::Single) => total + 1,
            Some(Style::Double) => total + 2,
            Some(Style::Triple) => total + 3,
            Some(Style::Quadruple) => total + 4,
            Some(Style::Aromatic) => total + 1,
            Some(Style::Up) => total + 1,
            Some(Style::Down) => total + 1,
            None => total + 1
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn none_none() {
        let bonds = vec![
            Bond { tid: 0, style: None },
            Bond { tid: 1, style: None }
        ];

        assert_eq!(valence(&bonds), 2);
    }

    #[test]
    fn single_single() {
        let bonds = vec![
            Bond { tid: 0, style: Some(Style::Single) },
            Bond { tid: 1, style: Some(Style::Single) }
        ];

        assert_eq!(valence(&bonds), 2);
    }

    #[test]
    fn single_double() {
        let bonds = vec![
            Bond { tid: 0, style: Some(Style::Single) },
            Bond { tid: 0, style: Some(Style::Double) }
        ];

        assert_eq!(valence(&bonds), 3);
    }

    #[test]
    fn single_triple() {
        let bonds = vec![
            Bond { tid: 0, style: Some(Style::Single) },
            Bond { tid: 0, style: Some(Style::Triple) }
        ];

        assert_eq!(valence(&bonds), 4);
    }

    #[test]
    fn single_quadruple() {
        let bonds = vec![
            Bond { tid: 0, style: Some(Style::Single) },
            Bond { tid: 0, style: Some(Style::Quadruple) }
        ];

        assert_eq!(valence(&bonds), 5);
    }

    #[test]
    fn double_up() {
        let bonds = vec![
            Bond { tid: 0, style: Some(Style::Double) },
            Bond { tid: 0, style: Some(Style::Up) }
        ];

        assert_eq!(valence(&bonds), 3);
    }

    #[test]
    fn double_down() {
        let bonds = vec![
            Bond { tid: 0, style: Some(Style::Double) },
            Bond { tid: 0, style: Some(Style::Down) }
        ];

        assert_eq!(valence(&bonds), 3);
    }

    #[test]
    fn single_aromatic() {
        let bonds = vec![
            Bond { tid: 0, style: Some(Style::Single) },
            Bond { tid: 0, style: Some(Style::Aromatic) }
        ];

        assert_eq!(valence(&bonds), 2);
    }
}