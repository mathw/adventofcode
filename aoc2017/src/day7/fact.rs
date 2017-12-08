use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fact {
    pub name: String,
    pub weight: u32,
    pub underneath: HashSet<String>,
}

impl Fact {
    pub fn new(name: String, weight: u32, underneath: HashSet<String>) -> Fact {
        Fact { name, weight, underneath }
    }
}