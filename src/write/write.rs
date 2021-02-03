use crate::tree::{ Atom, Link, Target };
use super::{
    write_atom_kind::write_atom_kind, write_bond_kind::write_bond_kind,
    write_rnum::write_rnum
};

/// Returns a String representing the given tree. Performs a depth-first
/// traversal over the root atom. Branches are written in the order they occur
/// in the tree.
/// 
/// ```
/// use purr::read::{ read, Error };
/// use purr::write::write;
///
/// fn main() -> Result<(), Error> {
///     let root = read("c1ccccc1")?.root;
///
///     assert_eq!(write(&root), "c1ccccc1");
///
///     Ok(())
/// }
/// ```
pub fn write(root: &Atom) -> String {
    let mut stack = Vec::new();
    let mut out = String::new();

    write_atom_kind(&root.kind, &mut out);

    for (i, link) in root.links.iter().rev().enumerate() {
        stack.push(Unit::from(i, link, None))
    }

    while let Some(Unit { link, branch }) = stack.pop() {
        if let Some(Branch::Open) = branch {
            out.push('(')
        }

        match link {
            Link::Bond { kind, target } => {
                write_bond_kind(kind, &mut out);

                match target {
                    Target::Atom(target) => {
                        write_atom_kind(&target.kind, &mut out);
                        tail(branch, target, &mut out, &mut stack)
                    },
                    Target::Join(rnum) => {
                        // out.push_str(&rnum.to_string())
                        write_rnum(rnum, &mut out)
                    }
                }
            },
            Link::Split(target) => {
                out.push('.');
                write_atom_kind(&target.kind, &mut out);
                tail(branch, target, &mut out, &mut stack)
            }
        }
    }

    out
}

fn tail<'a>(
    branch: Option<Branch>,
    target: &'a Atom,
    out: &mut String,
    stack: & mut Vec<Unit<'a>>
) {
    if branch.is_some() && target.links.is_empty() {
        out.push(')')
    }

    for (i, link) in target.links.iter().rev().enumerate() {
        stack.push(Unit::from(i, link, branch.clone()))
    }
}

#[derive(Debug,PartialEq)]
struct Unit<'a> {
    link: &'a Link,
    branch: Option<Branch>
}

impl<'a> Unit<'a> {
    fn outside(link: &'a Link) -> Self {
        Self { link, branch: None }
    }

    fn open(link: &'a Link) -> Self {
        Self { link, branch: Some(Branch::Open) }
    }

    fn inside(link: &'a Link) -> Self {
        Self { link, branch: Some(Branch::Inside) }
    }

    fn from(index: usize, link: &Link, branch: Option<Branch>) -> Unit {
        match branch {
            Some(Branch::Open) |
            Some(Branch::Inside) => {
                if index == 0 {
                    Unit::inside(link)
                } else {
                    Unit::open(link)
                }
            },
            None => {
                if index == 0 {
                    Unit::outside(link)
                } else {
                    match link {
                        Link::Bond { target, .. } => match target {
                            Target::Atom(_) => Unit::open(link),
                            Target::Join(_) => Unit::outside(link)
                        },
                        Link::Split(_) => Unit::open(link)
                    }
                }
            }
        }
    }
}

#[derive(Debug,PartialEq,Clone)]
enum Branch {
    Open,
    Inside
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
    fn methane_bare() {
        let root = read("C").unwrap().root;

        assert_eq!(write(&root), "C")
    }

    #[test]
    fn methyl_radical() {
        let root = read("[CH3]").unwrap().root;

        assert_eq!(write(&root), "[CH3]")
    }

    #[test]
    fn methyl_cation() {
        let root = read("[CH3+]").unwrap().root;

        assert_eq!(write(&root), "[CH3+]")
    }

    #[test]
    fn methane_map() {
        let root = read("[CH4:42]").unwrap().root;

        assert_eq!(write(&root), "[CH4:42]")
    }

    #[test]
    fn methane_hydrate() {
        let root = read("C.O").unwrap().root;

        assert_eq!(write(&root), "C.O")
    }

    #[test]
    fn methanol() {
        let root = read("CO").unwrap().root;

        assert_eq!(write(&root), "CO")
    }

    #[test]
    fn dihalomethane() {
        let root = read("FCCl").unwrap().root;

        assert_eq!(write(&root), "FCCl")
    }

    #[test]
    fn dihalomethane_branched() {
        let root = read("C(F)Cl").unwrap().root;

        assert_eq!(write(&root), "C(F)Cl")
    }

    #[test]
    fn trihalomethane_branched() {
        let root = read("C(F)(Cl)Br").unwrap().root;

        assert_eq!(write(&root), "C(F)(Cl)Br")
    }

    #[test]
    fn fluoroethanol_branched() {
        let root = read("C(CF)O").unwrap().root;

        assert_eq!(write(&root), "C(CF)O")
    }

    #[test]
    fn oxirane() {
        let root = read("C1OC1").unwrap().root;

        assert_eq!(write(&root), "C1OC1")
    }

    #[test]
    fn oxirane_left_single() {
        let root = read("C-1OC1").unwrap().root;

        assert_eq!(write(&root), "C-1OC1")
    }

    #[test]
    fn oxirane_right_single() {
        let root = read("C1OC-1").unwrap().root;

        assert_eq!(write(&root), "C1OC-1")
    }

    #[test]
    fn bicyclobutane() {
        let root = read("C12CC1C2").unwrap().root;

        assert_eq!(write(&root), "C12CC1C2")
    }

    #[test]
    fn benzene_aromatic_atoms() {
        let root = read("c1ccccc1").unwrap().root;

        assert_eq!(write(&root), "c1ccccc1")
    }

    #[test]
    fn benzene_aromatic_bonds() {
        let root = read("C:1:C:C:C:C:C:C1").unwrap().root;

        assert_eq!(write(&root), "C:1:C:C:C:C:C:C1")
    }

    #[test]
    fn open_after_open() {
        let root = read("C(C(O)F)C").unwrap().root;

        assert_eq!(write(&root), "C(C(O)F)C")
    }

    #[test]
    fn open_after_inside() {
        let root = read("C(CC(O)F)C").unwrap().root;

        assert_eq!(write(&root), "C(CC(O)F)C")
    }

    #[test]
    fn split_after_open() {
        let root = read("C(.O)C").unwrap().root;

        assert_eq!(write(&root), "C(.O)C") 
    }

    #[test]
    fn split_after_inside() {
        let root = read("C(.OCl)C").unwrap().root;

        assert_eq!(write(&root), "C(.OCl)C") 
    }

    #[test]
    fn bond_after_open() {
        let root = read("C(-O[SiH3])C").unwrap().root;

        assert_eq!(write(&root), "C(-O[SiH3])C") 
    }
}