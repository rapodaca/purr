#[derive(Debug,PartialEq)]
pub enum Error {
    IncompatibleJoin(usize, usize)
}