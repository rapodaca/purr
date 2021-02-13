use std::fmt;

use crate::tree::Follower;
use crate::parts::{ AtomKind, BondKind };
use crate::tree::walk;
use super::{ Rnum, Atom };

#[derive(Debug,PartialEq)]
pub struct Writer {
    stack: Vec<String>
}

impl Writer {
    pub fn new() -> Self {
        Self {
            stack: Vec::new()
        }
    }

    pub fn write(root: &Atom) -> String {
        let mut writer = Writer::new();

        walk(root, &mut writer);
    
        writer.to_string()
    }
}

impl fmt::Display for Writer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.stack.join(""))
    }
}

impl Follower for Writer {
    fn root(&mut self, root: &AtomKind) {
        if self.stack.is_empty() {
            self.stack.push(root.to_string())
        } else {
            self.stack.push(".".to_string() + &root.to_string())
        }
    }

    fn extend(&mut self, bond_kind: &BondKind, atom_kind: &AtomKind) {
        self.stack.push(bond_kind.to_string() + &atom_kind.to_string())
    }

    fn join(&mut self, bond_kind: &BondKind, rnum: &Rnum) {
        let last = self.stack.last_mut().expect("last");

        last.push_str(&(bond_kind.to_string() + &rnum.to_string()))
    }

    fn pop(&mut self, depth: usize) {
        if depth >= self.stack.len() {
            panic!("overpop")
        }

        let chain = self.stack.split_off(self.stack.len() - depth);
        let last = self.stack.last_mut().expect("last");

        last.push_str(&("(".to_string() + &chain.join("") + ")"))
    }
}

#[cfg(test)]
mod write {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn p1() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);

        assert_eq!(writer.to_string(), "*")
    }

    #[test]
    fn p2() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);
        writer.extend(&BondKind::Single, &AtomKind::Star);

        assert_eq!(writer.to_string(), "*-*")
    }

    #[test]
    fn p1_p1() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);
        writer.root(&AtomKind::Star);

        assert_eq!(writer.to_string(), "*.*")
    }

    #[test]
    fn p3() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);
        writer.extend(&BondKind::Single, &AtomKind::Star);
        writer.extend(&BondKind::Single, &AtomKind::Star);

        assert_eq!(writer.to_string(), "*-*-*")
    }

    #[test]
    fn p3_branched() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);
        writer.extend(&BondKind::Single, &AtomKind::Star);
        writer.pop(1);
        writer.extend(&BondKind::Double, &AtomKind::Star);

        assert_eq!(writer.to_string(), "*(-*)=*")
    }

    #[test]
    fn c3() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);
        writer.join(&BondKind::Single, &Rnum::R1);
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.join(&BondKind::Single, &Rnum::R1);

        assert_eq!(writer.to_string(), "*-1**-1")
    }

    #[test]
    fn nested_branch() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.extend(&BondKind::Single, &AtomKind::Star);
        writer.pop(1);
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.pop(2);
        writer.extend(&BondKind::Double, &AtomKind::Star);

        assert_eq!(writer.to_string(), "*(*(-*)*)=*")
    }
}