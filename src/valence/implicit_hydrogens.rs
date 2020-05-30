use crate::mol::Mol;
use crate::mol::Atom;
use crate::mol::Bond;
use crate::mol::Style;
use super::error::Error;
use super::valence_gap::valence_gap;

pub fn implicit_hydrogens(id: &usize, mol: &Mol) -> Result<Option<u8>, Error> {
    let atom = match mol.atoms.get(*id) {
        Some(atom) => atom,
        None => return Err(Error::UnknownID)
    };

    if atom.hcount.is_some() {
        return Ok(None);
    }

    let bonds = mol.bonds.get(*id).unwrap();
    
    match valence_gap(&atom.element, bonds) {
        Some(gap) => {
            if gap > 0 && is_aromatic(atom, bonds) {
                Ok(Some(gap - 1))
            } else {
                Ok(Some(gap))
            }
        },
        None => Ok(None)
    }
}

fn is_aromatic(atom: &Atom, bonds: &Vec<Bond>) -> bool {
    if atom.aromatic {
        return true;
    }

    for bond in bonds.iter() {
        if let Some(style) = bond.style {
            if style == Style::Aromatic {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mol::Element;

    #[test]
    fn unknown_id() {
        let mol = Mol {
            atoms: vec![ ],
            bonds: vec![ vec![ ] ]
        };

        assert_eq!(implicit_hydrogens(&0, &mol), Err(Error::UnknownID));
    }

    #[test]
    fn some_hcount() {
        let mol = Mol {
            atoms: vec![
                Atom { hcount: Some(1), ..Default::default() }
            ],
            bonds: vec![ vec![ ] ]
        };

        assert_eq!(implicit_hydrogens(&0, &mol).unwrap(), None);
    }

    #[test]
    fn methane_carbon() {
        let mol = Mol {
            atoms: vec![
                Atom { ..Default::default() }
            ],
            bonds: vec![ vec![ ] ]
        };

        assert_eq!(implicit_hydrogens(&0, &mol).unwrap(), Some(4));
    }

    #[test]
    fn aromatic_azete_nitrogen() {
        let mol = Mol {
            atoms: vec![
                Atom {
                    element: Element::N, aromatic: true, ..Default::default()
                },
                Atom { aromatic: true, ..Default::default() },
                Atom { aromatic: true, ..Default::default() },
                Atom { aromatic: true, ..Default::default() }
            ],
            bonds: vec![
                vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 3, style: None }
                ],
                vec![
                    Bond { tid: 0, style: None },
                    Bond { tid: 2, style: None }
                ],
                vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 3, style: None }
                ],
                vec![
                    Bond { tid: 0, style: None },
                    Bond { tid: 2, style: None }
                ]
            ]
        };

        assert_eq!(implicit_hydrogens(&0, &mol).unwrap(), Some(0));
    }
}