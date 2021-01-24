use std::ptr;

use crate::tree::{ Atom, Link, Target };
use super::Follower;

pub fn walk(root: &Atom, follower: &mut dyn Follower) {
    let mut stack = Vec::new();
    let mut path = vec![ root ];

    for link in root.links.iter().rev() {
        stack.push((root, link))
    }

    loop {
        let (root, link) = match stack.pop() {
            Some(entry) => entry,
            None => break
        };

        loop {
            let last = *path.last().expect("last in path");

            if ptr::eq(last, root) {
                break;
            } else {
                path.pop();
                follower.back();
            }
        }

        match link {
            Link::Bond { kind, target } => {
                match target {
                    Target::Atom(target) => {
                        follower.extend(kind, &target.kind);
                        path.push(target);

                        for next_link in target.links.iter().rev() {
                            stack.push((target, next_link))
                        }
                    },
                    Target::Join(rnum) => {
                        follower.join(kind, *rnum);
                        follower.back();
                    }
                }
            },
            Link::Split(target) => {
                follower.split(&target.kind);
                path.push(target);

                for next_link in target.links.iter().rev() {
                    stack.push((target, next_link))
                }
            }
        }
    }
}