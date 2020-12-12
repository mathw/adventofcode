use super::seating::{Seat, Seating};
use std::time::Duration;

pub trait SolutionFinder {
    fn find_solution_visually<D>(&self, drawfunc: &mut D, delay: Option<Duration>) -> (Self, usize)
    where
        D: FnMut(&Self, usize) -> (),
        Self: Sized;

    fn find_solution(&self) -> (Self, usize)
    where
        Self: Sized;
}

pub trait SeatIterable {
    fn iterate_seat(&self, x: usize, y: usize, index: usize) -> Seat;
    fn seating(&self) -> &Seating;
    fn seating_mut(&mut self) -> &mut Seating;
}
