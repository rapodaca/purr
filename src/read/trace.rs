use std::ops::Range;

/// A traced used with `read`. The `atom`, `bonds`, and `rnums` attributes
/// map the corresponding ordered element id to a cursor or range. For
/// example, access the first atom cursor range with
/// `Trace.atoms[0]`, the second bond cursor with `Trace.bonds[1]` the
/// second rnum cursor range with `Trace.bonds[2]`, and so on.
#[derive(Debug,PartialEq)]
pub struct Trace {
    pub atoms: Vec<Range<usize>>,
    pub bonds: Vec<usize>,
    pub rnums: Vec<Range<usize>>
}

impl Trace {
    pub fn new() -> Self {
        Self {
            atoms: Vec::new(),
            bonds: Vec::new(),
            rnums: Vec::new()
        }
    }
}