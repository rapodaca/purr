/// An error resulting from depth-first traversal of a graph
/// representation.
#[derive(Debug,PartialEq)]
pub enum Error {
    HalfBond(usize, usize),
    DuplicateBond(usize, usize),
    UnknownTarget(usize, usize),
    IncompatibleBond(usize, usize),
    Loop(usize)
}