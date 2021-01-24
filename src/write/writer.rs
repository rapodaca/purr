use crate::parts::{ AtomKind, BondKind };
use crate::tree::Follower;
use super::{ Cell, write_atom_kind, write_bond_kind };

#[derive(Debug,PartialEq)]
pub struct Writer {
    cells: Vec<Cell>
}

impl Writer {
    pub fn new(root: &AtomKind) -> Self {
        Self {
            cells: vec![ Cell::new(write_atom_kind(root), true) ]
        }
    }

    pub fn write(mut self) -> String {
        loop {
            let last = self.cells.pop().expect("last cell");

            if let Some(previous) = self.cells.last_mut() {
                previous.merge(last)
            } else {
                break last.to_string()
            }
        }
    }
}

impl Follower for Writer {
    fn extend(&mut self, bond_kind: &BondKind, atom_kind: &AtomKind) {
        let mut string = write_bond_kind(bond_kind);

        string.push_str(&write_atom_kind(atom_kind));
        self.cells.push(Cell::new(string, true))
    }

    fn join(&mut self, bond_kind: &BondKind, rnum: u16) {
        let mut string = write_bond_kind(bond_kind);

        if rnum > 10 {
            string.push('%');
        }
        
        string.push_str(&rnum.to_string());
        self.cells.push(Cell::new(string, false))
    }

    fn split(&mut self, atom_kind: &AtomKind) {
        let mut string = String::from(".");

        string.push_str(&write_atom_kind(atom_kind));
        self.cells.push(Cell::new(string, true));
    }

    fn back(&mut self) {
        let last = self.cells.pop().expect("last cell");
        let previous = self.cells.last_mut().expect("previous cell");

        previous.merge(last);
    }
}

#[cfg(test)]
mod write {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn p1() {
        let writer = Writer::new(&AtomKind::Star);

        assert_eq!(writer.write(), "*")
    }

    #[test]
    fn p2() {
        let mut writer = Writer::new(&AtomKind::Star);

        writer.extend(&BondKind::Elided, &AtomKind::Star);

        assert_eq!(writer.write(), "**")
    }

    #[test]
    fn p2_single() {
        let mut writer = Writer::new(&AtomKind::Star);

        writer.extend(&BondKind::Single, &AtomKind::Star);

        assert_eq!(writer.write(), "*-*")
    }

    #[test]
    fn p1_p1() {
        let mut writer = Writer::new(&AtomKind::Star);

        writer.split(&AtomKind::Star);

        assert_eq!(writer.write(), "*.*")
    }

    #[test]
    fn p3() {
        let mut writer = Writer::new(&AtomKind::Star);

        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.extend(&BondKind::Elided, &AtomKind::Star);

        assert_eq!(writer.write(), "***")
    }

    #[test]
    fn p3_branched() {
        let mut writer = Writer::new(&AtomKind::Star);

        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.back();
        writer.extend(&BondKind::Elided, &AtomKind::Star);

        assert_eq!(writer.write(), "*(*)*")
    }

    #[test]
    fn c3() {
        let mut writer = Writer::new(&AtomKind::Star);

        writer.join(&BondKind::Elided, 1);
        writer.back();
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.join(&BondKind::Elided, 1);

        assert_eq!(writer.write(), "*1**1")
    }

    #[test]
    fn c3_digit_double() {
        let mut writer = Writer::new(&AtomKind::Star);

        writer.join(&BondKind::Elided, 42);
        writer.back();
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.join(&BondKind::Elided, 42);

        assert_eq!(writer.write(), "*%42**%42")
    }

    #[test]
    fn c3_double_left() {
        let mut writer = Writer::new(&AtomKind::Star);

        writer.join(&BondKind::Double, 1);
        writer.back();
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.join(&BondKind::Elided, 1);

        assert_eq!(writer.write(), "*=1**1")
    }

    #[test]
    fn c3_join_last() {
        let mut writer = Writer::new(&AtomKind::Star);

        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.join(&BondKind::Elided, 1);
        writer.back();
        writer.back();
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.join(&BondKind::Elided, 1);

        assert_eq!(writer.write(), "*(*1)*1")
    }

    #[test]
    fn s3() {
        let mut writer = Writer::new(&AtomKind::Star);

        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.back();
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.back();
        writer.extend(&BondKind::Elided, &AtomKind::Star);

        assert_eq!(writer.write(), "*(*)(*)*")
    }

    #[test]
    fn p4() {
        let mut writer = Writer::new(&AtomKind::Star);

        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.extend(&BondKind::Elided, &AtomKind::Star);

        assert_eq!(writer.write(), "****")
    }

    #[test]
    fn p4_branched() {
        let mut writer = Writer::new(&AtomKind::Star);

        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.back();
        writer.back();
        writer.extend(&BondKind::Elided, &AtomKind::Star);

        assert_eq!(writer.write(), "*(**)*")
    }

    #[test]
    fn nested_branch() {
        let mut writer = Writer::new(&AtomKind::Star);

        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.back();
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.back();
        writer.back();
        writer.extend(&BondKind::Elided, &AtomKind::Star);

        assert_eq!(writer.write(), "*(*(*)*)*")
    }
}