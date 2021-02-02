use super::Atom;

/// Distinguishes two different targets that may appear at the end of a
/// `Link`: those to an atom; and those to a ring index (rnum).
#[derive(Debug,PartialEq)]
pub enum Target {
    Join(u16),
    Atom(Atom)
}