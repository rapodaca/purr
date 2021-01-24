use crate::tree::{ Atom, walk };
use super::Writer;

pub fn write(root: &Atom) -> String {
    let mut writer = Writer::new(&root.kind);

    walk(root, &mut writer);

    writer.write()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::read::read;
    use super::*;

    #[test]
    fn p1() {
        let root = read("*").unwrap().root;

        assert_eq!(write(&root), "*")
    }

    #[test]
    fn p2() {
        let root = read("**").unwrap().root;

        assert_eq!(write(&root), "**")
    }

    #[test]
    fn p2_triple() {
        let root = read("*#*").unwrap().root;

        assert_eq!(write(&root), "*#*")
    }

    #[test]
    fn p3_branched() {
        let root = read("*(*)*").unwrap().root;

        assert_eq!(write(&root), "*(*)*")
    }

    #[test]
    fn c3() {
        let root = read("*(**1)1").unwrap().root;

        assert_eq!(write(&root), "*(**1)1")
    }
}