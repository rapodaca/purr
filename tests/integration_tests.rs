use purr::read::{ read, Error };
use purr::valence::implicit_hydrogens;
use purr::mol::{ Mol, Atom, Bond, Element };

#[test]
fn read_invalid_atom() {
    let mol = read(&"X");

    assert_eq!(mol.err(), Some(Error::InvalidCharacter(0)));
}

#[test]
fn read_ethanol_bracket() -> Result<(), Error> {
    let mol = read(&"OC[CH3]")?;

    assert_eq!(mol, Mol {
        atoms: vec![
            Atom { element: Element::O, ..Default::default() },
            Atom { ..Default::default() },
            Atom { hcount: Some(3), charge: Some(0), ..Default::default() }
        ],
        bonds: vec![
            vec![ Bond { tid: 1, style: None } ],
            vec![ Bond { tid: 0, style: None }, Bond { tid: 2, style: None } ],
            vec![ Bond { tid: 1, style: None } ]
        ]
    });

    assert_eq!(implicit_hydrogens(&mol.atoms[0], &mol.bonds[0]), Some(1));
    assert_eq!(implicit_hydrogens(&mol.atoms[1], &mol.bonds[1]), Some(2));
    assert_eq!(implicit_hydrogens(&mol.atoms[2], &mol.bonds[2]), None);
    
    Ok(())
}