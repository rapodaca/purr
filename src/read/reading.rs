use crate::tree;

/// A root tree node and a `trace` vector. Trace is a zero-indexed mapping
/// of an atom index to a cursor position. For example, to access the cursor
/// position for the third atom, use `trace[2]`.
/// 
/// The purpose of trace is to
/// allow the reporting of semantic errors (such as valence) using the
/// cursor position of the corresponding atom.
#[derive(Debug,PartialEq)]
pub struct Reading {
    pub root: tree::Atom,
    pub trace: Vec<usize>
}