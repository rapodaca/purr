#[derive(Debug,PartialEq)]
pub enum Error {
    Join(usize, usize),
    Rnum(usize)
}