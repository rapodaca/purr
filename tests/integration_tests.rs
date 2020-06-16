use purr::read::{ read, Error };
use purr::valence::implicit_hydrogens;
use purr::mol::{ Mol, Atom, Bond, Element };

#[test]
fn read_invalid_atom() {
    let mol = read(&"X");

    assert_eq!(mol.err(), Some(Error::InvalidCharacter(0)));
}

#[test]
fn read_ethanol_bracket() {
    let mol = read(&"OC[CH3]").unwrap();

    assert_eq!(mol, Mol {
        atoms: vec![
            Atom { element: Element::O, ..Default::default() },
            Atom { ..Default::default() },
            Atom { hcount: Some(3), ..Default::default() }
        ],
        bonds: vec![
            vec![ Bond { tid: 1, style: None } ],
            vec![ Bond { tid: 0, style: None }, Bond { tid: 2, style: None } ],
            vec![ Bond { tid: 1, style: None } ]
        ]
    });
    assert_eq!(implicit_hydrogens(&0, &mol), Ok(Some(1)));
    assert_eq!(implicit_hydrogens(&1, &mol), Ok(Some(2)));
    assert_eq!(implicit_hydrogens(&2, &mol), Ok(None));
}