use purr::read::read;
use purr::valence::implicit_hydrogens;
use purr::mol::{ Mol, Atom, Bond, Element };

#[test]
fn test_add() {
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