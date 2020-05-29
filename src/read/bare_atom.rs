pub use super::error::Error;
pub use crate::mol::Atom;
pub use crate::util::Scanner;
pub use crate::mol::Element;

pub fn bare_atom(scanner: &mut Scanner) -> Result<Option<Atom>, Error> {
    match scanner.peek() {
        Some('b') => pop_and_go(scanner, Element::B, true),
        Some('c') => pop_and_go(scanner, Element::C, true),
        Some('n') => pop_and_go(scanner, Element::N, true),
        Some('o') => pop_and_go(scanner, Element::O, true),
        Some('p') => pop_and_go(scanner, Element::P, true),
        Some('s') => pop_and_go(scanner, Element::S, true),
        Some('B') => {
            scanner.pop();

            match scanner.peek() {
                Some('r') => pop_and_go(scanner, Element::Br, false),
                _ => Ok(Some(Atom {
                    element: Element::B, ..Default::default()
                }))
            }
        },
        Some('C') => {
            scanner.pop();

            match scanner.peek() {
                Some('l') => pop_and_go(scanner, Element::Cl, false),
                _ => Ok(Some(Atom {
                    element: Element::C, ..Default::default()
                }))
            }
        },
        Some('N') => pop_and_go(scanner, Element::N, false),
        Some('O') => pop_and_go(scanner, Element::O, false),
        Some('P') => pop_and_go(scanner, Element::P, false),
        Some('S') => pop_and_go(scanner, Element::S, false),
        Some('F') => pop_and_go(scanner, Element::F, false),
        Some('I') => pop_and_go(scanner, Element::I, false),
        _ => Ok(None)
    }
}

fn pop_and_go(
    scanner: &mut Scanner, element: Element, aromatic: bool
) -> Result<Option<Atom>, Error> {
    scanner.pop();

    Ok(Some(Atom { element, aromatic, ..Default::default() }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aromatic_carbon() {
        let mut scanner = Scanner::new(&"c");
        let result = bare_atom(&mut scanner);

        assert_eq!(result, Ok(Some(Atom {
            element: Element::C, aromatic: true, ..Default::default()
        })));
        assert_eq!(scanner.pop(), None);
    }

    #[test]
    fn aliphatic_carbon() {
        let mut scanner = Scanner::new(&"C");
        let result = bare_atom(&mut scanner);

        assert_eq!(result, Ok(Some(Atom {
            element: Element::C, ..Default::default()
        })));
        assert_eq!(scanner.pop(), None);
    }

    #[test]
    fn aliphatic_chlorine() {
        let mut scanner = Scanner::new(&"Cl");
        let result = bare_atom(&mut scanner);

        assert_eq!(result, Ok(Some(Atom {
            element: Element::Cl, ..Default::default()
        })));
        assert_eq!(scanner.pop(), None);
    }
}