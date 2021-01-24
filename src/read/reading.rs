use crate::tree;

#[derive(Debug,PartialEq)]
pub struct Reading {
    pub root: tree::Atom,
    pub trace: Vec<usize>
}