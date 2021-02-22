/// An error occuring during the conversion of a tree to a graph.
#[derive(Debug,PartialEq)]
pub enum Error {
    UnbalancedRnum(usize),
    IncompatibleJoin(usize, usize)
}