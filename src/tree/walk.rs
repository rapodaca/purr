use crate::tree::{ Atom, Link, Target };
use super::Follower;

pub fn walk<F: Follower>(root: &Atom, follower: &mut F) {
    follower.root(&root.kind);

    let mut stack = Vec::new();
    let mut chain = Chain::new();

    for link in root.links.iter().rev() {
        stack.push((chain.id, link))
    }

    chain.next();

    while let Some((sid, link)) = stack.pop() {
        let depth = chain.prune(sid);

        if depth > 0 {
            follower.pop(depth)
        }

        match link {
            Link::Bond { kind, target } => match target {
                Target::Atom(target) => {
                    for link in target.links.iter().rev() {
                        stack.push((chain.id, link))
                    }

                    follower.extend(kind, &target.kind);
                    chain.next()
                },
                Target::Join(rnum) => {
                    follower.join(kind, rnum)
                }
            },
            Link::Split(target) => {
                for link in target.links.iter().rev() {
                    stack.push((chain.id, link))
                }

                follower.root(&target.kind);
                chain.next()
            }
        }
    }
}

struct Chain {
    id: usize,
    path: Vec<usize>
}

impl Chain {
    fn new() -> Self {
        Self {
            id: 0,
            path: Vec::new()
        }
    }

    fn next(&mut self) {
        self.path.push(self.id);

        self.id += 1
    }

    fn prune(&mut self, id: usize) -> usize {
        let mut count = 0;

        while let Some(last) = self.path.pop() {
            if last == id {
                self.path.push(id);

                return count
            }

            count += 1;
        }

        unreachable!("prune id not found")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::read::read;
    use crate::tree::Writer;
    use super::*;

    #[test]
    fn p1() {
        let tree = read("*", None).unwrap();
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.to_string(), "*")
    }

    #[test]
    fn p2() {
        let tree = read("**", None).unwrap();
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.to_string(), "**")
    }

    #[test]
    fn p1_p1() {
        let tree = read("*.*", None).unwrap();
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.to_string(), "*.*")
    }

    #[test]
    fn p3() {
        let tree = read("*-*=*", None).unwrap();
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.to_string(), "*-*=*")
    }

    #[test]
    fn p3_branched() {
        let tree = read("*(-*)=*", None).unwrap();
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.to_string(), "*(-*)=*")
    }

    #[test]
    fn c3() {
        let tree = read("*-1**-1", None).unwrap();
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.to_string(), "*-1**-1")
    }

    #[test]
    fn p4_branched() {
        let tree = read("*(-**)=*", None).unwrap();
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.to_string(), "*(-**)=*")
    }

    #[test]
    fn butterfly() {
        let tree = read("*12**2*1", None).unwrap();
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.to_string(), "*12**2*1")
    }

    #[test]
    fn nested_branch() {
        let tree = read("*(*(-*)*)=*", None).unwrap();
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.to_string(), "*(*(-*)*)=*")
    }

    #[test]
    fn caffeine() {
        let tree = read("CN1C=NC2=C1C(=O)N(C(=O)N2C)C", None).unwrap();
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.to_string(), "CN1C=NC2=C1C(=O)N(C(=O)N2C)C")
    }
}