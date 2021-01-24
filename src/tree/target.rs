use super::Atom;

#[derive(Debug,PartialEq)]
pub enum Target {
    Join(u16),
    Atom(Atom)
}