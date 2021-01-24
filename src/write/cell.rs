use std::fmt;

#[derive(Debug,PartialEq)]
pub struct Cell {
    string: String,
    wrap: bool,
    cursor: Option<usize>
}

impl Cell {
    pub fn new(string: String, wrap: bool) -> Self {
        Self {
            string,
            wrap,
            cursor: None
        }
    }

    pub fn merge(&mut self, other: Cell) {
        if let Some(cursor) = self.cursor.take() {
            self.string.insert(cursor, '(');
            self.string.push(')');
        }

        if other.wrap {
            self.cursor.replace(self.string.len());
        }

        self.string.push_str(&other.string)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.string.fmt(f)
    }
}

#[cfg(test)]
mod to_string {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn wrap() {
        let mut cell = Cell::new(String::from("C"), true);

        cell.merge(Cell::new(String::from("Br"), false));

        assert_eq!(cell.to_string(), "CBr")
    }

    #[test]
    fn wrap_wrap() {
        let mut cell = Cell::new(String::from("C"), true);

        cell.merge(Cell::new(String::from("F"), true));
        cell.merge(Cell::new(String::from("Br"), true));

        assert_eq!(cell.to_string(), "C(F)Br")
    }

    #[test]
    fn wrap_wrap_wrap() {
        let mut cell = Cell::new(String::from("C"), true);

        cell.merge(Cell::new(String::from("F"), true));
        cell.merge(Cell::new(String::from("Cl"), true));
        cell.merge(Cell::new(String::from("Br"), true));

        assert_eq!(cell.to_string(), "C(F)(Cl)Br")
    }

    #[test]
    fn no_wrap() {
        let mut cell = Cell::new(String::from("C"), true);

        cell.merge(Cell::new(String::from("%42"), false));

        assert_eq!(cell.to_string(), "C%42")
    }

    #[test]
    fn no_wrap_wrap() {
        let mut cell = Cell::new(String::from("C"), true);

        cell.merge(Cell::new(String::from("%42"), false));
        cell.merge(Cell::new(String::from("Br"), true));

        assert_eq!(cell.to_string(), "C%42Br")
    }

    #[test]
    fn no_wrap_no_wrap() {
        let mut cell = Cell::new(String::from("C"), true);

        cell.merge(Cell::new(String::from("%42"), false));
        cell.merge(Cell::new(String::from("%13"), false));

        assert_eq!(cell.to_string(), "C%42%13")
    }
}