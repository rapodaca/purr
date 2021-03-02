use std::collections::{ HashMap, BinaryHeap };
use std::collections::hash_map::Entry;
use std::cmp::{ Ord, Ordering };
use std::hash::{ Hash, Hasher };
use std::convert::TryInto;

use crate::feature::Rnum;

#[derive(Eq,PartialEq,PartialOrd)]
struct Index(u16);

impl Ord for Index {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

#[derive(Debug,Eq)]
struct Pair(usize, usize);

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        (self.0.eq(&other.0) && self.1.eq(&other.1)) ||
        (self.0.eq(&other.1) && self.1.eq(&other.0))
    }
}

impl Hash for Pair {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        (self.0 + self.1).hash(hasher)
    }
}

pub struct JoinPool {
    counter: u16,
    borrowed: HashMap<Pair, u16>,
    replaced: BinaryHeap<Index>
}

impl JoinPool {
    pub fn new() -> Self {
        Self {
            counter: 1,
            borrowed: HashMap::new(),
            replaced: BinaryHeap::new()
        }
    }

    pub fn hit(&mut self, sid: usize, tid: usize) -> Rnum {
        let next = match self.replaced.pop() {
            Some(next) => next.0,
            None => {
                let next = self.counter;
                self.counter += 1;

                next
            }
        };

        match self.borrowed.entry(Pair(sid, tid)) {
            Entry::Occupied(occupied) => {
                let result = occupied.remove();

                self.replaced.push(Index(result));

                result.try_into().expect("rnum")
            },
            Entry::Vacant(vacant) => {
                vacant.insert(next);

                next.try_into().expect("rnum")
            }
        }
    }
}

#[cfg(test)]
mod pair {
    use super::*;

    #[test]
    fn hashmap() {
        let mut map = HashMap::new();

        map.insert(Pair(0, 1), 0);
        map.insert(Pair(1, 0), 1);

        assert_eq!(map.len(), 1)
    }
}

#[cfg(test)]
mod hit {
    use super::*;

    #[test]
    fn unknown() {
        let mut pool = JoinPool::new();

        assert_eq!(pool.hit(1, 2), Rnum::R1);
        assert_eq!(pool.hit(1, 5), Rnum::R2);
        assert_eq!(pool.hit(13, 42), Rnum::R3)
    }

    #[test]
    fn known() {
        let mut pool = JoinPool::new();

        assert_eq!(pool.hit(0, 1), Rnum::R1);
        assert_eq!(pool.hit(1, 0), Rnum::R1)
    }

    #[test]
    fn unknown_with_one_returned() {
        let mut pool = JoinPool::new();

        assert_eq!(pool.hit(0, 1), Rnum::R1);
        assert_eq!(pool.hit(1, 0), Rnum::R1);
        assert_eq!(pool.hit(13, 42), Rnum::R1)
    }

    #[test]
    fn unknown_with_two_returned() {
        let mut pool = JoinPool::new();

        assert_eq!(pool.hit(0, 1), Rnum::R1);
        assert_eq!(pool.hit(1, 3), Rnum::R2);
        assert_eq!(pool.hit(2, 4), Rnum::R3);
        assert_eq!(pool.hit(3, 1), Rnum::R2);
        assert_eq!(pool.hit(1, 0), Rnum::R1);
        assert_eq!(pool.hit(3, 5), Rnum::R1)
    }
}