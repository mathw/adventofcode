//! I nicked this from Reddit and then updated it for Rust 1.x and then me-ified it.
//! It provides an iterator over all possible pairs in a source iterator

pub struct Pairs<T: Copy, U: Iterator<Item = T> + Clone> {
    head: Option<T>,
    tail: U,
    next: U,
}

impl<T: Copy, U: Iterator<Item = T> + Clone> Pairs<T, U> {
    pub fn new(mut iter: U) -> Pairs<T, U> {
        let head = iter.next();
        Pairs {
            head: head,
            tail: iter.clone(),
            next: iter,
        }
    }
}

impl<T: Copy, U: Iterator<Item = T> + Clone> Iterator for Pairs<T, U> {
    type Item = (T, T);

    fn next(&mut self) -> Option<(T, T)> {
        self.head.and_then(|a| {
            match self.tail.next() {
                Some(b) => Some((a, b)),
                None => {
                    match self.next.next() {
                        Some(new_head) => {
                            self.head = Some(new_head);
                            self.tail = self.next.clone();
                            self.next()
                        }
                        _ => None,
                    }
                }
            }
        })
    }
}
