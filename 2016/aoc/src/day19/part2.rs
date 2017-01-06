#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Elf {
    number: u32,
    presents: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Circle {
    elves: Vec<Elf>,
}


impl Elf {
    fn new(number: u32) -> Elf {
        Elf {
            number: number,
            presents: 1,
        }
    }
}


impl Circle {
    fn new(elves: u32) -> Circle {
        let mut elves_vec = Vec::new();
        for e in 1..(elves + 1) {
            elves_vec.push(Elf::new(e));
        }

        Circle { elves: elves_vec }
    }

    fn index_of_elf(&self, number: u32) -> Option<usize> {
        let r = self.elves.binary_search_by_key(&number, |elf| elf.number);
        match r {
            Ok(index) => Some(index),
            Err(_) => None,
        }
    }

    fn elf_across_from(&self, number: u32) -> Option<Elf> {
        if let Some(reference_elf_index) = self.index_of_elf(number) {
            let elves_in_circle = self.elves.len();
            let halfway = elves_in_circle / 2;
            let across_index = (reference_elf_index + halfway) % elves_in_circle;
            if across_index == reference_elf_index {
                return None;
            }
            return self.elves.get(across_index).map(|e| e.clone());
        }

        None
    }

    fn remove_elf(&self, number: u32) -> Option<Circle> {
        if let Some(i) = self.index_of_elf(number) {
            let mut elves = self.elves.clone();
            elves.remove(i);
            Some(Circle { elves: elves })
        } else {
            None
        }
    }

    fn add_presents_to_elf(&self, number: u32, presents: u32) -> Option<Circle> {
        if let Some(i) = self.index_of_elf(number) {
            let mut elves = self.elves.clone();
            elves[i].presents += presents;
            Some(Circle { elves: elves })
        } else {
            None
        }
    }

    fn take_presents(&self, taker: u32, takee: &Elf) -> Option<Circle> {
        self.add_presents_to_elf(taker, takee.presents)
            .and_then(|circle| circle.remove_elf(takee.number))
    }

    fn elf_after(&self, elf: u32) -> Option<Elf> {
        #[cfg(test)]
        println!("elf_after {}: vec is {:?}", elf, self.elves);
        self.elves
            .iter()
            .enumerate() // add indexes
            .skip_while(|&(_, e)| e.number != elf) // skip all the elves not us
            .next() // get us
            .map(|(i, _)| (i + 1) % self.elves.len()) // index of the elf after us
            .and_then(|i| self.elves.get(i)) // return that elf
            .map(|e| e.clone())
    }

    fn run_circle(&self) -> Option<Elf> {
        let mut next_elf = self.elves.iter().next().unwrap().clone();
        #[cfg(test)]
        println!("First elf will be {}", next_elf.number);
        let mut current_circle = self.clone();

        loop {
            match current_circle.elf_across_from(next_elf.number) {
                Some(victim) => {
                    #[cfg(test)]
                    println!("Elf across from {} is {}", next_elf.number, victim.number);
                    match current_circle.take_presents(next_elf.number, &victim) {
                        Some(new_circle) => {
                            #[cfg(test)]
                            println!("Taken presents from {}", victim.number);
                            current_circle = new_circle;
                            match current_circle.elf_after(next_elf.number) {
                                Some(elf) => {
                                    #[cfg(test)]
                                    println!("Next elf will be {}", elf.number);
                                    next_elf = elf;
                                }
                                None => break,
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }

        #[cfg(test)]
        println!("Returning {}", next_elf.number);
        Some(next_elf)
    }
}

pub fn do_day19_part2() {
    let circle = Circle::new(3005290);

    let elf = circle.run_circle();

    println!("{:?}", elf);
}


#[test]
fn test_index_of_elf() {
    let circle = Circle::new(5);

    assert_eq!(circle.index_of_elf(1), Some(0));
    assert_eq!(circle.index_of_elf(2), Some(1));
    assert_eq!(circle.index_of_elf(3), Some(2));
    assert_eq!(circle.index_of_elf(4), Some(3));
    assert_eq!(circle.index_of_elf(5), Some(4));
    assert_eq!(circle.index_of_elf(6), None);
}

#[test]
fn test_elf_across_from() {
    let circle = Circle::new(5);

    assert_eq!(circle.elf_across_from(1).unwrap().number, 3);
    assert_eq!(circle.elf_across_from(3).unwrap().number, 5);
    assert!(circle.elf_across_from(6).is_none());
}

#[test]
fn test_five() {
    let circle = Circle::new(5);

    let elf = circle.run_circle();
    assert!(elf.is_some());
    if let Some(elf) = elf {
        assert_eq!(elf.number, 2);
    } else {
        assert!(false);
    }
}

#[test]
fn test_next_elf() {
    let circle = Circle::new(5);

    let elf = circle.elves.iter().next().unwrap();
    let next_elf = circle.elf_after(elf.number).unwrap();

    assert_eq!(elf.number, 1);
    assert_eq!(next_elf.number, 2);

    let next_elf = circle.elf_after(5).unwrap();
    assert_eq!(next_elf.number, 1);
}
