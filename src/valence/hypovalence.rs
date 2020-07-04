use crate::mol::{ Atom, Bond };
use super::{ targets, valence, Error };

/// Returns the difference between the next-nearest target valence and
/// the bond order sum. Returns None if valence targets are not defined.
/// Includes hcount, but disregards aromatic flag.
/// Returns error if target valences are available but none match.
pub fn hypovalence(atom: &Atom, bonds: &Vec<Bond>) -> Result<Option<u8>, Error> {
    match targets(atom) {
        Some(targets) => {
            let sum = valence(bonds);

            for target in targets {
                if target >= sum {
                    match atom.hcount {
                        Some(hcount) => return Ok(Some(target - sum - hcount)),
                        None => return Ok(Some(target - sum))
                    }
                }
            }
            
            Err(Error::UnmatchableValence)
        },
        None => Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mol::{ Style, Element };

    #[test]
    fn boron() {
        let atom = Atom {
            element: Element::B, ..Default::default()
        };

        assert_eq!(hypovalence(&atom, &vec![ ]), Ok(Some(3)));
    }

    #[test]
    fn boron_none() {
        let atom = Atom {
            element: Element::B, ..Default::default()
        };

        assert_eq!(hypovalence(&atom, &vec![
            Bond { tid: 1, style: None }
        ]), Ok(Some(2)));
    }

    #[test]
    fn boron_single_none_none() {
        let atom = Atom {
            element: Element::B, ..Default::default()
        };

        assert_eq!(hypovalence(&atom, &vec![
            Bond { tid: 1, style: Some(Style::Single) },
            Bond { tid: 2, style: None },
            Bond { tid: 3, style: None }
        ]), Ok(Some(0)));
    }

    #[test]
    fn carbon_one_hydrogen() {
        let atom = Atom {
            element: Element::C, hcount: Some(1), ..Default::default()
        };

        assert_eq!(hypovalence(&atom, &vec![ ]), Ok(Some(3)));
    }

    #[test]
    fn carbon_single_single() {
        let atom = Atom {
            element: Element::C, ..Default::default()
        };

        assert_eq!(hypovalence(&atom, &vec![
            Bond { tid: 1, style: Some(Style::Single) },
            Bond { tid: 2, style: Some(Style::Single) },
        ]), Ok(Some(2)));
    }

    #[test]
    fn carbon_double_up() {
        let atom = Atom {
            element: Element::C, ..Default::default()
        };

        assert_eq!(hypovalence(&atom, &vec![
            Bond { tid: 1, style: Some(Style::Double) },
            Bond { tid: 2, style: Some(Style::Up) },
        ]), Ok(Some(1)));
    }

    #[test]
    fn carbon_aromatic_aromatic_single() {
        let atom = Atom {
            element: Element::C, ..Default::default()
        };

        assert_eq!(hypovalence(&atom, &vec![
            Bond { tid: 1, style: Some(Style::Aromatic) },
            Bond { tid: 2, style: Some(Style::Aromatic) },
            Bond { tid: 3, style: Some(Style::Single) },
        ]), Ok(Some(1)));
    }

    #[test]
    fn nitrogen_single() {
        let atom = Atom {
            element: Element::N, ..Default::default()
        };

        assert_eq!(hypovalence(&atom, &vec![
            Bond { tid: 1, style: Some(Style::Single) },
        ]), Ok(Some(2)));
    }

    #[test]
    fn nitrogen_double_none_none() {
        let atom = Atom {
            element: Element::N, ..Default::default()
        };

        assert_eq!(hypovalence(&atom, &vec![
            Bond { tid: 1, style: Some(Style::Double) },
            Bond { tid: 2, style: None },
            Bond { tid: 3, style: None },
        ]), Ok(Some(1)));
    }

    #[test]
    fn titanium_none_none() {
        let atom = Atom {
            element: Element::Ti, ..Default::default()
        };

        assert_eq!(hypovalence(&atom, &vec![
            Bond { tid: 2, style: None },
            Bond { tid: 3, style: None },
        ]), Ok(None));
    }
}