use super::traits::SeatIterable;
use std::{thread, time::Duration};

pub fn iterate_until_stable<D, S>(
    seating: &S,
    drawfunc: &mut D,
    delay: Option<Duration>,
) -> (S, usize)
where
    D: (FnMut(&S, usize) -> ()),
    S: SeatIterable + Eq + Clone,
{
    let mut previous = seating.clone();
    let mut next = iterate(seating);
    let mut iterations = 1;

    while previous != next {
        drawfunc(&next, iterations);
        previous = next;
        next = iterate(&previous);
        iterations += 1;
        if let Some(d) = delay {
            thread::sleep(d);
        }
    }

    (next, iterations)
}

pub fn iterate<S: SeatIterable + Clone>(seating: &S) -> S {
    let mut new = (*seating).clone();

    for y in 0..seating.seating().height {
        for x in 0..seating.seating().width {
            let index = seating.seating().to_index_unchecked(x, y);
            new.seating_mut().seats[index] = seating.iterate_seat(x, y, index);
        }
    }
    new
}
