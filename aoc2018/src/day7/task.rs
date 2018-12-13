use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Task {
    name: char,
    duration: u32,
}

impl Task {
    pub fn new(name: char, duration: u32) -> Task {
        Task {
            name,
            duration: duration,
        }
    }

    pub fn name(&self) -> char {
        self.name
    }

    pub fn duration(&self) -> u32 {
        self.duration
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Task) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Task) -> Ordering {
        self.name.cmp(&other.name)
    }
}
