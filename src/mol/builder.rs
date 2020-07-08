use std::collections::HashMap;

use super::Atom;
use super::Bond;
use super::Nub;
use super::Style;
use super::Error;
use super::match_styles::match_styles;

pub struct Builder {
    atoms: Vec<Atom>,
    style: Option<Style>,
    stack: Vec<usize>,
    root: usize,
    cuts: HashMap<u8, Cut>
}

impl Builder {
    pub fn new(root: Nub ) -> Self {
       Self {
           atoms: vec![ Atom::new(root) ],
           style: None,
           stack: vec![ ],
           root: 0,
           cuts: HashMap::new()
       }
    }

    pub fn root(&mut self, nub: Nub) {
        self.add_atom(nub);
    }

    pub fn bond(&mut self, style: Style) {
        self.style.replace(style);
    }

    pub fn cut(&mut self, rnum: u8) -> Result<(), Error> {
        match self.cuts.remove(&rnum) {
            Some(cut) => self.close_cut(cut),
            None => Ok(self.open_cut(rnum))
        }
    }

    pub fn extend(&mut self, nub: Nub) {
        let sid = self.root;
        let tid = self.atoms.len();
        let style = self.style.take();

        self.add_atom(nub);
        self.add_bond(sid, tid, style);
    }

    pub fn open(&mut self) {
        self.stack.push(self.root);
    }

    pub fn close(&mut self) {
        self.root = self.stack.pop().unwrap();
    }

    pub fn to_atoms(self) -> Result<Vec<Atom>, Error> {
        if !self.stack.is_empty() {
            return Err(Error::OpenBranches(self.stack));
        }

        if !self.cuts.is_empty() {
            let rnums = self.cuts.keys().cloned().collect::<Vec<_>>();

            return Err(Error::OpenCycles(rnums));
        }

        Ok(self.atoms)
    }

    fn add_atom(&mut self, nub: Nub) {
        self.root = self.atoms.len();

        self.atoms.push(Atom { nub, bonds: vec![ ] });
    }

    fn add_bond(&mut self, sid: usize, tid: usize, style: Option<Style>) {
        self.atoms[sid].bonds.push(Bond { tid, style });

        let reversed_style = match style {
            Some(Style::Up) => Some(Style::Down),
            Some(Style::Down) => Some(Style::Up),
            _ => style
        };

        self.atoms[tid].bonds.push(Bond {
            tid: sid, style: reversed_style
        });
    }

    fn open_cut(&mut self, rnum: u8) {
        let id = self.root;
        let bonds = &mut self.atoms[id].bonds;
        let index = bonds.len();
        let style = self.style.take();

        self.cuts.insert(rnum, Cut { id, index });
        bonds.push(Bond { tid: id, style: style });
    }

    fn close_cut(&mut self, cut: Cut) -> Result<(), Error> {
        let sid = self.root;
        let tid = cut.id;
        let bond = self.atoms[tid].bonds.get_mut(cut.index).unwrap();

        match match_styles(bond.style, self.style.take()) {
            Some((left, right)) => {
                std::mem::replace(bond, Bond {
                    tid: sid, style: left
                });
                self.atoms[sid].bonds.push(Bond {
                    tid, style: right
                });

                Ok(())
            },
            None => Err(Error::MismatchedStyle(sid, tid))
        }
    }
}

struct Cut {
    id: usize,
    index: usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mol::Element;

    #[test]
    fn cut_given_mismatch() {
        let c = Nub { element: Element::C, ..Default::default() };
        let mut builder = Builder::new(c);

        builder.bond(Style::Double);
        assert_eq!(builder.cut(1), Ok(()));
        builder.extend(c);
        builder.extend(c);
        builder.bond(Style::Single);

        assert_eq!(builder.cut(1), Err(Error::MismatchedStyle(2, 0)));
    }

    #[test]
    fn to_atoms_given_open_branch() {
        let c = Nub { element: Element::C, ..Default::default() };
        let mut builder = Builder::new(c);

        builder.open();

        assert_eq!(builder.to_atoms(), Err(Error::OpenBranches(vec![ 0 ])));
    }

    #[test]
    fn to_atoms_given_open_cycle() {
        let c = Nub { element: Element::C, ..Default::default() };
        let mut builder = Builder::new(c);

        assert_eq!(builder.cut(1), Ok(()));
        assert_eq!(builder.to_atoms(), Err(Error::OpenCycles(vec![ 1 ])));
    }

    #[test]
    fn to_atoms_given_methane() {
        let c = Nub { ..Default::default() };
        let builder = Builder::new(c);

        assert_eq!(builder.to_atoms(), Ok(vec![
            Atom { nub: c, bonds: vec![ ] }
        ]));
    }

    #[test]
    fn to_atoms_given_ethane() {
        let c = Nub { ..Default::default() };
        let mut builder = Builder::new(c);

        builder.extend(c);

        assert_eq!(builder.to_atoms(), Ok(vec![
            Atom {
                nub: c, bonds: vec![ Bond { tid: 1, style: None } ]
            },
            Atom {
                nub: c, bonds: vec![ Bond { tid: 0, style: None } ]
            }
        ]));
    }

    #[test]
    fn to_atoms_given_ethane_up() {
        let c = Nub { ..Default::default() };
        let mut builder = Builder::new(c);

        builder.bond(Style::Up);
        builder.extend(c);

        assert_eq!(builder.to_atoms(), Ok(vec![
            Atom {
                nub: c, bonds: vec![ Bond { tid: 1, style: Some(Style::Up) } ]
            },
            Atom {
                nub: c, bonds: vec![ Bond { tid: 0, style: Some(Style::Down) } ]
            }
        ]));
    }

    #[test]
    fn to_atoms_given_methane_hydrate() {
        let c = Nub { ..Default::default() };
        let o = Nub { element: Element::O, ..Default::default() };
        let mut builder = Builder::new(c);

        builder.root(o);

        assert_eq!(builder.to_atoms(), Ok(vec![
            Atom {
                nub: c, bonds: vec![ ]
            },
            Atom {
                nub: o, bonds: vec![ ]
            }
        ]));
    }

    #[test]
    fn to_atoms_given_propene() {
        let c = Nub { ..Default::default() };
        let mut builder = Builder::new(c);

        builder.bond(Style::Double);
        builder.extend(c);
        builder.extend(c);

        assert_eq!(builder.to_atoms(), Ok(vec![
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 1, style: Some(Style::Double) }
                ]
            },
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 0, style: Some(Style::Double) },
                    Bond { tid: 2, style: None }
                ]
            },
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 1, style: None }
                ]
            }
        ]));
    }

    #[test]
    fn to_atoms_given_branch() {
        let c = Nub { ..Default::default() };
        let mut builder = Builder::new(c);

        builder.open();
        builder.extend(c);
        builder.close();
        builder.extend(c);

        assert_eq!(builder.to_atoms(), Ok(vec![
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 2, style: None }
                ]
            },
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 0, style: None }
                ]
            },
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 0, style: None }
                ]
            }
        ]));
    }

    #[test]
    fn to_atoms_given_sequential_branch() {
        let c = Nub { ..Default::default() };
        let mut builder = Builder::new(c);

        builder.open();
        builder.extend(c);
        builder.close();
        builder.open();
        builder.extend(c);
        builder.close();
        builder.extend(c);

        assert_eq!(builder.to_atoms(), Ok(vec![
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 2, style: None },
                    Bond { tid: 3, style: None }
                ]
            },
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 0, style: None }
                ]
            },
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 0, style: None }
                ]
            },
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 0, style: None }
                ]
            }
        ]));
    }

    #[test]
    fn to_atoms_given_double_nested_branch() {
        let c = Nub { ..Default::default() };
        let mut builder = Builder::new(c);

        builder.open();
        builder.extend(c);
        builder.open();
        builder.extend(c);
        builder.close();
        builder.extend(c);
        builder.close();
        builder.extend(c);

        assert_eq!(builder.to_atoms(), Ok(vec![
                Atom {
                    nub: c, bonds: vec![
                        Bond { tid: 1, style: None },
                        Bond { tid: 4, style: None }
                    ]
                },
                Atom {
                    nub: c, bonds: vec![
                        Bond { tid: 0, style: None },
                        Bond { tid: 2, style: None },
                        Bond { tid: 3, style: None }
                    ]
                },
                Atom {
                    nub: c, bonds: vec![
                        Bond { tid: 1, style: None }
                    ]
                },
                Atom {
                    nub: c, bonds: vec![
                        Bond { tid: 1, style: None }
                    ]
                },
                Atom {
                    nub: c, bonds: vec![
                        Bond { tid: 0, style: None }
                    ]
                }
        ]));
    }

    #[test]
    fn to_atoms_given_cyclopropane() {
        let c = Nub { ..Default::default() };
        let mut builder = Builder::new(c);

        assert_eq!(builder.cut(1), Ok(()));
        builder.extend(c);
        builder.extend(c);
        assert_eq!(builder.cut(1), Ok(()));

        assert_eq!(builder.to_atoms(), Ok(vec![
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 2, style: None },
                    Bond { tid: 1, style: None }
                ]
            },
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 0, style: None },
                    Bond { tid: 2, style: None }
                ]
            },
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 0, style: None }
                ]
            }
        ]));
    }

    #[test]
    fn to_atoms_given_cyclopropene_first() {
        let c = Nub { ..Default::default() };
        let mut builder = Builder::new(c);

        builder.bond(Style::Double);
        assert_eq!(builder.cut(1), Ok(()));
        builder.extend(c);
        builder.extend(c);
        assert_eq!(builder.cut(1), Ok(()));

        assert_eq!(builder.to_atoms(), Ok(vec![
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 2, style: Some(Style::Double) },
                    Bond { tid: 1, style: None }
                ]
            },
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 0, style: None },
                    Bond { tid: 2, style: None }
                ]
            },
            Atom {
                nub: c, bonds: vec![
                    Bond { tid: 1, style: None },
                    Bond { tid: 0, style: Some(Style::Double) }
                ]
            }
        ]));
    }
}