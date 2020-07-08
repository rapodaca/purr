#[derive(PartialEq, Debug)]
pub enum Error {
    MismatchedStyle(usize, usize),
    OpenBranches(Vec<usize>),
    OpenCycles(Vec<u8>)
}