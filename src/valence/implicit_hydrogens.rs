use crate::mol::{ Atom, Bond, Style };
use super::hypovalence;

/// Returns the implicit hydrogen count for the atom, or None if the
/// atom isn't a member of the valence model.
pub fn implicit_hydrogens(atom: &Atom, bonds: &Vec<Bond>) -> Option<u8> {
    match atom.hcount {
        Some(_) => None,
        None => {
            match hypovalence(atom, bonds) {
                Some(delta) => {
                    if delta > 0 && aromatic(atom, bonds) {
                        Some(delta - 1)
                    } else {
                        Some(delta)
                    }
                },
                None => None
            }
        }
    }
}

fn aromatic(atom: &Atom, bonds: &Vec<Bond>) -> bool {
    if atom.aromatic {
        return true;
    }

    bonds.iter().any(|bond| {
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

    #[test]
    fn default() {
        let atom = Atom { ..Default::default() };

        assert_eq!(implicit_hydrogens(&atom, &vec![ ]), Some(4));
    }

    #[test]
    fn zero_h() {
        let atom = Atom { hcount: Some(0), ..Default::default() };

        assert_eq!(implicit_hydrogens(&atom, &vec![ ]), None);
    }

    #[test]
    fn one_h() {
        let atom = Atom { hcount: Some(2), ..Default::default() };

        assert_eq!(implicit_hydrogens(&atom, &vec![ ]), None);
    }

    #[test]
    fn aromatic_flag() {
        let atom = Atom { aromatic: true, ..Default::default() };

        assert_eq!(implicit_hydrogens(&atom, &vec![ ]), Some(3));
    }

    #[test]
    fn aromatic_bond() {
        let atom = Atom { ..Default::default() };

        assert_eq!(implicit_hydrogens(&atom, &vec![
            Bond { tid: 1, style: Some(Style::Aromatic) }
        ]), Some(2));
    }

    #[test]
    fn nonmember_element() {
        let atom = Atom { element: Element::Sn, ..Default::default() };

        assert_eq!(implicit_hydrogens(&atom, &vec![ ]), None);
    }
}