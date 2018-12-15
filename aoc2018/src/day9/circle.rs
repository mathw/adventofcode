use std::fmt;
use std::ops::{Index, IndexMut};
use std::usize;

// based very heavily on https://www.reddit.com/r/rust/comments/7zsy72/writing_a_doubly_linked_list_in_rust_is_easy/
// but I'm not using the slab crate because I can handle the memory wastage and not using it saves me
// 100ms

pub struct Circle {
    vec: Vec<Marble>,
    current_marble: Pointer,
    next_value: u32,
}

struct Marble {
    value: u32,
    next: Pointer,
    prev: Pointer,
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Pointer(usize);

impl Pointer {
    #[inline]
    fn null() -> Pointer {
        Pointer(usize::MAX)
    }

    #[inline]
    fn is_null(&self) -> bool {
        *self == Pointer::null()
    }
}

impl Index<Pointer> for Circle {
    type Output = Marble;

    fn index(&self, index: Pointer) -> &Self::Output {
        &self.vec[index.0]
    }
}

impl IndexMut<Pointer> for Circle {
    fn index_mut(&mut self, index: Pointer) -> &mut Self::Output {
        &mut self.vec[index.0]
    }
}

impl Circle {
    pub fn new(marbles: usize) -> Circle {
        let mut circle = Circle {
            vec: Vec::with_capacity(marbles),
            current_marble: Pointer::null(),
            next_value: 1,
        };

        circle.vec.push(Marble {
            value: 0,
            prev: Pointer::null(),
            next: Pointer::null(),
        });

        let n = Pointer(0);
        circle[n].prev = n;
        circle[n].next = n;
        circle.current_marble = n;

        circle
    }

    fn insert_before(&mut self, node: Pointer) -> Pointer {
        let prev = self[node].prev;
        self.vec.push(Marble {
            value: self.next_value,
            prev: prev,
            next: node,
        });
        let n = Pointer(self.vec.len() - 1);

        self[prev].next = n;
        self[node].prev = n;
        self.current_marble = n;

        self.next_value += 1;

        n
    }

    fn remove(&mut self, node: Pointer) -> u32 {
        let prev = self[node].prev;
        let next = self[node].next;

        if prev == node {
            panic!("Can't remove! Circle will become empty!");
        }

        self[prev].next = next;
        self[next].prev = prev;

        self.current_marble = next;

        let removed_value = self.vec[node.0].value;
        // yes it leaves the value in place, but nobody's ever going to point to it again
        // in this puzzle scenario it doesn't matter much anyway

        removed_value
    }

    fn remove_seven_left(&mut self) -> u32 {
        let mut to_remove = self.current_marble;
        for _ in 0..7 {
            to_remove = self[to_remove].prev;
        }
        self.remove(to_remove)
    }

    fn insert_two_right(&mut self) {
        let mut new_location = self.current_marble;
        for _ in 0..2 {
            new_location = self[new_location].next;
        }

        self.insert_before(new_location);
    }

    pub fn add_new_marble(&mut self) -> u32 {
        let new_value = self.next_value;
        if new_value % 23 == 0 {
            // craziness!
            let removed_score = self.remove_seven_left();
            self.next_value += 1;
            removed_score + new_value
        } else {
            self.insert_two_right();
            0
        }
    }

    #[cfg(test)]
    fn all_marbles(&self) -> Vec<u32> {
        let mut result = Vec::new();

        let mut n = self.current_marble;
        result.push(self[n].value);
        n = self[n].next;

        while n != self.current_marble {
            result.push(self[n].value);
            n = self[n].next;
        }

        result
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut n = self.current_marble;
        write!(f, " ({}) ", self[n].value)?;
        n = self[n].next;

        while n != self.current_marble {
            write!(f, " {} ", self[n].value)?;
            n = self[n].next;
        }

        Ok(())
    }
}

#[test]
fn add_marbles() {
    let mut circle = Circle::new(5);
    circle.add_new_marble();
    circle.add_new_marble();
    circle.add_new_marble();
    circle.add_new_marble();

    assert_eq!(circle.all_marbles(), vec![4, 2, 1, 3, 0]);
}

#[test]
fn add_twenty_three_marbles() {
    let mut circle = Circle::new(23);
    let mut score: u32 = 0;

    println!("{}", circle);

    for _ in 0..23 {
        score = circle.add_new_marble();

        println!("{}", circle);
    }

    assert_eq!(score, 32);
}
