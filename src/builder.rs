use std::collections::HashMap;

use crate::Atom;
use crate::Bond;
use crate::Mol;
use crate::Style;
use crate::match_styles::match_styles;

pub struct Builder {
    atoms: Vec<Atom>, 
    bonds: Vec<Vec<Bond>>,
    style: Option<Style>,
    stack: Vec<usize>,
    root: usize,
    cuts: HashMap<u8, Cut>
}

impl Builder {
    pub fn new(root: Atom) -> Self {
        Self {
            atoms: vec![ root ],
            bonds: vec![ vec![ ] ],
            style: None,
            stack: vec![ ],
            root: 0,
            cuts: HashMap::new()
        }
    }

    pub fn root(&mut self, atom: Atom) {
        self.add_atom(atom);
    }

    pub fn extend(&mut self, atom: Atom) {
        let sid = self.root;
        let tid = self.atoms.len();
        let style = self.style.take();

        self.add_atom(atom);
        self.add_bond(sid, tid, style);
    }

    pub fn bond(&mut self, style: Style) {
        self.style.replace(style);
    }

    pub fn open(&mut self) {
        self.stack.push(self.root);
    }

    pub fn close(&mut self) {
        self.root = self.stack.pop().unwrap();
    }

    pub fn cut(&mut self, rnum: u8) -> Result<(), Error> {
        match self.cuts.remove(&rnum) {
            Some(cut) => self.close_cut(cut),
            None => Ok(self.open_cut(rnum))
        }
    }

    pub fn to_mol(self) -> Result<Mol, Error> {
        if !self.stack.is_empty() {
            return Err(Error::OpenBranch);
        }

        if !self.cuts.is_empty() {
            return Err(Error::OpenCycle);
        }

        Ok(Mol {
            atoms: self.atoms,
            bonds: self.bonds
        })
    }

    fn add_atom(&mut self, atom: Atom) {
        self.root = self.atoms.len();

        self.bonds.push(vec![ ]);
        self.atoms.push(atom);
    }

    fn add_bond(&mut self, sid: usize, tid: usize, style: Option<Style>) {
        self.bonds.get_mut(sid).unwrap().push(Bond { tid, style });

        let reversed_style = match style {
            Some(Style::Up) => Some(Style::Down),
            Some(Style::Down) => Some(Style::Up),
            _ => style
        };

        self.bonds.get_mut(tid).unwrap().push(Bond {
            tid: sid, style: reversed_style
        });
    }

    fn open_cut(&mut self, rnum: u8) {
        let id = self.root;
        let bonds = self.bonds.get_mut(id).unwrap();
        let index = bonds.len();
        let style = self.style.take();

        self.cuts.insert(rnum, Cut { id, index });
        bonds.push(Bond { tid: id, style: style });
    }

    fn close_cut(&mut self, cut: Cut) -> Result<(), Error> {
        let sid = self.root;
        let tid = cut.id;
        let bond = self.bonds.get_mut(tid).unwrap().get_mut(cut.index).unwrap();

        match match_styles(bond.style, self.style.take()) {
            Some((left, right)) => {
                std::mem::replace(bond, Bond {
                    tid: sid, style: left
                });
                self.bonds.get_mut(sid).unwrap().push(Bond {
                    tid, style: right
                });

                Ok(())
            },
            None => Err(Error::MismatchedStyle)
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    MismatchedStyle,
    OpenBranch,
    OpenCycle
}

struct Cut {
    id: usize,
    index: usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Element;

    #[test]
    fn cut_given_mismatch() {
        let c = Atom { element: Element::C, ..Default::default() };
        let mut builder = Builder::new(c);

        builder.bond(Style::Double);
        assert_eq!(builder.cut(1), Ok(()));
        builder.extend(c);
        builder.extend(c);
        builder.bond(Style::Single);

        assert_eq!(builder.cut(1), Err(Error::MismatchedStyle));
    }

    #[test]
    fn to_mol_given_open_branch() {
        let c = Atom { element: Element::C, ..Default::default() };
        let mut builder = Builder::new(c);

        builder.open();

        assert_eq!(builder.to_mol(), Err(Error::OpenBranch));
    }

    #[test]
    fn to_mol_given_open_cycle() {
        let c = Atom { element: Element::C, ..Default::default() };
        let mut builder = Builder::new(c);

        assert_eq!(builder.cut(1), Ok(()));

        assert_eq!(builder.to_mol(), Err(Error::OpenCycle));
    }

    #[test]
    fn to_mol_given_methane() {
        let c = Atom { element: Element::C, ..Default::default() };
        let builder = Builder::new(c);

        assert_eq!(builder.to_mol().unwrap(), Mol {
            atoms: vec![ c ],
            bonds: vec![ vec![ ] ] 
        });
    }

    #[test]
    fn to_mol_given_ethane() {
        let c = Atom { element: Element::C, ..Default::default() };
        let mut builder = Builder::new(c);

        builder.extend(c);

        assert_eq!(builder.to_mol().unwrap(), Mol {
            atoms: vec![ c, c ],
            bonds: vec![
                vec![ Bond { tid: 1, style: None } ],
                vec![ Bond { tid: 0, style: None } ]
            ]
        });
    }

    #[test]
    fn to_mol_given_ethane_up() {
        let c = Atom { element: Element::C, ..Default::default() };
        let mut builder = Builder::new(c);

        builder.bond(Style::Up);
        builder.extend(c);

        assert_eq!(builder.to_mol().unwrap(), Mol {
            atoms: vec![ c, c ],
            bonds: vec![
                vec![ Bond { tid: 1, style: Some(Style::Up) } ],
                vec![ Bond { tid: 0, style: Some(Style::Down) } ]
            ]
        });
    }

    #[test]
    fn to_mol_given_ethene() {
        let c = Atom { element: Element::C, ..Default::default() };
        let mut builder = Builder::new(c);

        builder.bond(Style::Double);
        builder.extend(c);

        assert_eq!(builder.to_mol().unwrap(), Mol {
            atoms: vec![ c, c ],
            bonds: vec![
                vec![ Bond { tid: 1, style: Some(Style::Double) } ],
                vec![ Bond { tid: 0, style: Some(Style::Double) } ]
            ]
        });
    }

    #[test]
    fn to_mol_given_methane_hydrate() {
        let c = Atom { element: Element::C, ..Default::default() };
        let o = Atom { element: Element::O, ..Default::default() };
        let mut builder = Builder::new(c);

        builder.root(o);

        assert_eq!(builder.to_mol().unwrap(), Mol {
            atoms: vec![ c, o ],
            bonds: vec![
                vec![ ],
                vec![ ]
            ]
        });
    }

    #[test]
    fn to_mol_given_propene() {
        let c = Atom { element: Element::C, ..Default::default() };
        let mut builder = Builder::new(c);

        builder.bond(Style::Double);
        builder.extend(c);
        builder.extend(c);

        assert_eq!(builder.to_mol().unwrap(), Mol {
            atoms: vec![ c, c, c ],
            bonds: vec![
                vec![
                    Bond { tid: 1, style: Some(Style::Double) }
                ],
                vec![
                    Bond { tid: 0, style: Some(Style::Double) },
                    Bond { tid: 2, style: None }
                ],
                vec![
                    Bond { tid: 1, style: None }
                ]
            ]
        });
    }

    #[test]
    fn to_mol_given_branch() {
        let c = Atom { element: Element::C, ..Default::default() };
        let mut builder = Builder::new(c);

        builder.open();
        builder.extend(c);
        builder.close();
        builder.extend(c);

        assert_eq!(builder.to_mol().unwrap(), Mol {
            atoms: vec![ c, c, c ],
            bonds: vec![
                vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 2, style: None }
                ],
                vec![
                    Bond { tid: 0, style: None }
                ],
                vec![
                    Bond { tid: 0, style: None }
                ]
            ]
        });
    }

    #[test]
    fn to_mol_given_sequential_branch() {
        let c = Atom { element: Element::C, ..Default::default() };
        let mut builder = Builder::new(c);

        builder.open();
        builder.extend(c);
        builder.close();
        builder.open();
        builder.extend(c);
        builder.close();
        builder.extend(c);

        assert_eq!(builder.to_mol().unwrap(), Mol {
            atoms: vec![ c, c, c, c ],
            bonds: vec![
                vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 2, style: None },
                    Bond { tid: 3, style: None }
                ],
                vec![
                    Bond { tid: 0, style: None }
                ],
                vec![
                    Bond { tid: 0, style: None }
                ],
                vec![
                    Bond { tid: 0, style: None }
                ]
            ]
        });
    }

    #[test]
    fn to_mol_given_double_nested_branch() {
        let c = Atom { element: Element::C, ..Default::default() };
        let mut builder = Builder::new(c);

        builder.open();
        builder.extend(c);
        builder.open();
        builder.extend(c);
        builder.close();
        builder.extend(c);
        builder.close();
        builder.extend(c);

        assert_eq!(builder.to_mol().unwrap(), Mol {
            atoms: vec![ c, c, c, c, c ],
            bonds: vec![
                vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 4, style: None }
                ],
                vec![
                    Bond { tid: 0, style: None },
                    Bond { tid: 2, style: None },
                    Bond { tid: 3, style: None }
                ],
                vec![
                    Bond { tid: 1, style: None }
                ],
                vec![
                    Bond { tid: 1, style: None }
                ],
                vec![
                    Bond { tid: 0, style: None }
                ]
            ]
        });
    }

    #[test]
    fn to_mol_given_cyclopropane() {
        let c = Atom { element: Element::C, ..Default::default() };
        let mut builder = Builder::new(c);

        assert_eq!(builder.cut(1), Ok(()));
        builder.extend(c);
        builder.extend(c);
        assert_eq!(builder.cut(1), Ok(()));

        assert_eq!(builder.to_mol().unwrap(), Mol {
            atoms: vec![ c, c, c ],
            bonds: vec![
                vec![
                    Bond { tid: 2, style: None },
                    Bond { tid: 1, style: None }
                ],
                vec![
                    Bond { tid: 0, style: None },
                    Bond { tid: 2, style: None }
                ],
                vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 0, style: None }
                ]
            ]
        });
    }

    #[test]
    fn to_mol_given_cyclopropene_first() {
        let c = Atom { element: Element::C, ..Default::default() };
        let mut builder = Builder::new(c);

        builder.bond(Style::Double);
        assert_eq!(builder.cut(1), Ok(()));
        builder.extend(c);
        builder.extend(c);
        assert_eq!(builder.cut(1), Ok(()));

        assert_eq!(builder.to_mol().unwrap(), Mol {
            atoms: vec![ c, c, c ],
            bonds: vec![
                vec![
                    Bond { tid: 2, style: Some(Style::Double) },
                    Bond { tid: 1, style: None }
                ],
                vec![
                    Bond { tid: 0, style: None },
                    Bond { tid: 2, style: None }
                ],
                vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 0, style: Some(Style::Double) }
                ]
            ]
        });
    }
}