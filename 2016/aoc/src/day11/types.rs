use std::fmt;
use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Element(String);

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Generator(Element);

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Microchip(Element);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Floor {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Clone)]
pub struct FloorContents {
    generators: HashSet<Generator>,
    microchips: HashSet<Microchip>,
}

enum ElevatorContents {
    Generator(Generator),
    Microchip(Microchip),
    Generators(Generator, Generator),
    Microchips(Microchip, Microchip),
    GeneratorAndMicrochip(Generator, Microchip),
    Nothing,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ValidMove {
    MoveGeneratorToFloor(Generator, Floor),
    MoveMicrochipToFloor(Microchip, Floor),
    MoveGeneratorAndMicrochipToFloor(Generator, Microchip, Floor),
    MoveTwoGeneratorsToFloor(Generator, Generator, Floor),
    MoveTwoMicrochipsToFloor(Microchip, Microchip, Floor),
}

pub struct ResearchCentre {
    floors: HashMap<Floor, FloorContents>,
    elevator_location: Floor,
    elevator_contents: ElevatorContents,
}

pub trait Compatible<T> {
    fn is_compatible_with(&self, other: &T) -> bool;
}

pub trait WithElement {
    fn element_name(&self) -> String;
}

impl Element {
    pub fn new(name: String) -> Element {
        Element(name)
    }
}

impl Generator {
    pub fn new(element: Element) -> Generator {
        Generator(element)
    }
}

impl Microchip {
    pub fn new(element: Element) -> Microchip {
        Microchip(element)
    }
}

impl ElevatorContents {
    fn contains_generator_type(&self, element: Element) -> bool {
        match self {
            &ElevatorContents::Generator(ref g) => g.element_name() == element.0,
            &ElevatorContents::Microchip(ref m) => false,
            &ElevatorContents::Generators(ref g1, ref g2) => {
                g1.element_name() == element.0 || g2.element_name() == element.0
            }
            &ElevatorContents::Microchips(ref m1, ref m2) => false,
            &ElevatorContents::GeneratorAndMicrochip(ref g, ref m) => g.element_name() == element.0,
            &ElevatorContents::Nothing => false,
        }
    }

    fn contains_generator_not_of_type(&self, element: Element) -> bool {
        match self {
            &ElevatorContents::Generator(ref g) => g.element_name() != element.0,
            &ElevatorContents::Microchip(ref m) => false,
            &ElevatorContents::Generators(ref g1, ref g2) => {
                g1.element_name() != element.0 && g2.element_name() != element.0
            }
            &ElevatorContents::Microchips(ref m1, ref m2) => false,
            &ElevatorContents::GeneratorAndMicrochip(ref g, ref m) => g.element_name() != element.0,
            &ElevatorContents::Nothing => false,
        }
    }
    fn contains_microchip_type(&self, element: Element) -> bool {
        match self {
            &ElevatorContents::Generator(ref g) => false,
            &ElevatorContents::Microchip(ref m) => m.element_name() == element.0,
            &ElevatorContents::Generators(ref g1, ref g2) => false,
            &ElevatorContents::Microchips(ref m1, ref m2) => {
                m1.element_name() == element.0 || m2.element_name() == element.0
            }
            &ElevatorContents::GeneratorAndMicrochip(ref g, ref m) => m.element_name() == element.0,
            &ElevatorContents::Nothing => false,
        }
    }

    fn contains_microchip_not_of_type(&self, element: Element) -> bool {
        match self {
            &ElevatorContents::Generator(ref g) => false,
            &ElevatorContents::Microchip(ref m) => m.element_name() != element.0,
            &ElevatorContents::Generators(ref g1, ref g2) => false,
            &ElevatorContents::Microchips(ref m1, ref m2) => {
                m1.element_name() != element.0 && m2.element_name() != element.0
            }
            &ElevatorContents::GeneratorAndMicrochip(ref g, ref m) => m.element_name() != element.0,
            &ElevatorContents::Nothing => false,
        }
    }
}

impl ResearchCentre {
    pub fn new() -> ResearchCentre {
        let mut floors = HashMap::new();
        floors.insert(Floor::First, FloorContents::new());
        floors.insert(Floor::Second, FloorContents::new());
        floors.insert(Floor::Third, FloorContents::new());
        floors.insert(Floor::Fourth, FloorContents::new());
        ResearchCentre {
            floors: floors,
            elevator_location: Floor::First,
            elevator_contents: ElevatorContents::Nothing,
        }
    }

    /// move the elevator to the given floor
    pub fn move_elevator_to(&mut self, floor: Floor) -> bool {
        if self.elevator_location == floor {
            return false;
        }

        self.elevator_location = floor;
        // TODO check that the elevator is allowed to move to this floor
        true
    }

    pub fn add_floor_contents(&mut self, floor: Floor, contents: FloorContents) {
        let mut c = self.floors.get_mut(&floor).unwrap();
        c.generators.extend(contents.generators.into_iter());
        c.microchips.extend(contents.microchips.into_iter());
    }

    pub fn is_everything_on_fourth_floor(&self) -> bool {
        self.floors.get(&Floor::First).unwrap().is_empty() &&
        self.floors.get(&Floor::Second).unwrap().is_empty() &&
        self.floors.get(&Floor::Third).unwrap().is_empty()
    }

    fn is_floor_empty(&self, floor: &Floor) -> bool {
        self.floors.get(floor).unwrap().is_empty()
    }

    pub fn get_all_valid_moves(&self) -> Vec<ValidMove> {
        let mut valid_moves = Vec::new();

        let current_floor_contents = self.floors.get(&self.elevator_location).unwrap();
        let other_floors = vec![Floor::Fourth, Floor::Third, Floor::Second, Floor::First]
            .into_iter()
            .filter(|f| *f != self.elevator_location)
            .collect::<Vec<_>>();

        for generator in current_floor_contents.generators.clone() {
            for floor in other_floors.clone().into_iter() {
                let m = ValidMove::MoveGeneratorToFloor(generator.clone(), floor);
                if self.is_valid_move(&m, &current_floor_contents) {
                    valid_moves.push(m);
                }
            }

            for generator2 in current_floor_contents.generators.clone() {
                if generator == generator2 {
                    continue;
                }

                for floor in other_floors.clone().into_iter() {
                    let m = ValidMove::MoveTwoGeneratorsToFloor(generator.clone(),
                                                                generator2.clone(),
                                                                floor);
                    if self.is_valid_move(&m, &current_floor_contents) {
                        valid_moves.push(m);
                    }
                }
            }
        }

        for microchip in current_floor_contents.microchips.clone() {
            for floor in other_floors.clone().into_iter() {
                let m = ValidMove::MoveMicrochipToFloor(microchip.clone(), floor);
                if self.is_valid_move(&m, &current_floor_contents) {
                    valid_moves.push(m);
                }
            }

            for microchip2 in current_floor_contents.microchips.clone() {
                if microchip == microchip2 {
                    continue;
                }

                for floor in other_floors.clone().into_iter() {
                    let m = ValidMove::MoveTwoMicrochipsToFloor(microchip.clone(),
                                                                microchip2.clone(),
                                                                floor);
                    if self.is_valid_move(&m, &current_floor_contents) {
                        valid_moves.push(m);
                    }
                }
            }

            for generator in current_floor_contents.generators.clone() {
                for floor in other_floors.clone().into_iter() {
                    let m = ValidMove::MoveGeneratorAndMicrochipToFloor(generator.clone(),
                                                                        microchip.clone(),
                                                                        floor);
                    if self.is_valid_move(&m, &current_floor_contents) {
                        valid_moves.push(m);
                    }
                }
            }
        }
        valid_moves
    }

    fn get_target_floor_contents(&self, floor: Floor) -> FloorContents {
        (*self.floors.get(&floor).unwrap()).clone()
    }

    fn get_all_floors_between(&self, origin: Floor, target: Floor) -> Vec<Floor> {
        match origin {
            Floor::First => {
                match target {
                    Floor::First => vec![],
                    Floor::Second => vec![Floor::Second],
                    Floor::Third => vec![Floor::Second, Floor::Third],
                    Floor::Fourth => vec![Floor::Second, Floor::Third, Floor::Fourth],
                }
            }
            Floor::Second => {
                match target {
                    Floor::First => vec![Floor::First],
                    Floor::Second => vec![],
                    Floor::Third => vec![Floor::Third],
                    Floor::Fourth => vec![Floor::Third, Floor::Fourth],
                }
            }
            Floor::Third => {
                match target {
                    Floor::First => vec![Floor::Second, Floor::First],
                    Floor::Second => vec![Floor::Second],
                    Floor::Third => vec![],
                    Floor::Fourth => vec![Floor::Fourth],
                }
            }
            Floor::Fourth => {
                match target {
                    Floor::First => vec![Floor::Third, Floor::Second, Floor::First],
                    Floor::Second => vec![Floor::Third, Floor::Second],
                    Floor::Third => vec![Floor::Third],
                    Floor::Fourth => vec![],
                }
            }
        }
    }

    fn get_floor_contents_between(&self, origin: Floor, target: Floor) -> Vec<FloorContents> {
        self.get_all_floors_between(origin, target)
            .into_iter()
            .map(|f| self.get_target_floor_contents(f))
            .collect()
    }

    fn is_valid_move(&self,
                     potential_move: &ValidMove,
                     current_floor_contents: &FloorContents)
                     -> bool {

        match potential_move {
            &ValidMove::MoveGeneratorToFloor(ref generator, floor) => {
                let floors_to_check =
                    self.get_floor_contents_between(self.elevator_location, floor);
                current_floor_contents.is_valid_without_generator(&generator) &&
                floors_to_check.iter().all(|f| f.can_accept_generator(generator))
            }
            &ValidMove::MoveMicrochipToFloor(ref microchip, floor) => {
                let floors_to_check =
                    self.get_floor_contents_between(self.elevator_location, floor);
                current_floor_contents.is_valid_without_microchip(&microchip) &&
                floors_to_check.iter().all(|f| f.can_accept_microchip(microchip))
            }
            &ValidMove::MoveGeneratorAndMicrochipToFloor(ref generator, ref microchip, floor) => {
                let floors_to_check =
                    self.get_floor_contents_between(self.elevator_location, floor);
                current_floor_contents.is_valid_without_microchip(&microchip) &&
                current_floor_contents.is_valid_without_generator(&generator) &&
                floors_to_check.iter()
                    .all(|f| f.can_accept_generator_and_microchip(&generator, &microchip))
            }
            &ValidMove::MoveTwoGeneratorsToFloor(ref generator1, ref generator2, floor) => {
                let floors_to_check =
                    self.get_floor_contents_between(self.elevator_location, floor);
                current_floor_contents.is_valid_without_generator(&generator1) &&
                current_floor_contents.is_valid_without_generator(&generator2) &&
                floors_to_check.iter().all(|f| f.can_accept_generators(&generator1, &generator2))
            }
            &ValidMove::MoveTwoMicrochipsToFloor(ref microchip1, ref microchip2, floor) => {
                let floors_to_check =
                    self.get_floor_contents_between(self.elevator_location, floor);
                current_floor_contents.is_valid_without_microchip(&microchip1) &&
                current_floor_contents.is_valid_without_microchip(&microchip2) &&
                floors_to_check.iter().all(|f| f.can_accept_microchips(&microchip1, &microchip2))
            }
        }
    }
}

impl FloorContents {
    fn new() -> FloorContents {
        FloorContents {
            generators: HashSet::new(),
            microchips: HashSet::new(),
        }
    }

    pub fn new_with_contents<GI: IntoIterator<Item = Generator>, MI: IntoIterator<Item = Microchip>>
        (generators: GI,
         microchips: MI)
         -> FloorContents {
        let mut gens = HashSet::new();
        for g in generators {
            gens.insert(g);
        }

        let mut chips = HashSet::new();
        for c in microchips {
            chips.insert(c);
        }

        FloorContents {
            generators: gens,
            microchips: chips,
        }
    }

    fn is_empty(&self) -> bool {
        self.generators.is_empty() && self.microchips.is_empty()
    }

    fn contains_element<'a, T, I>(things: I, element: &Element) -> bool
        where T: WithElement + 'a,
              I: IntoIterator<Item = &'a T>
    {
        things.into_iter().any(|t| t.element_name() == element.0)
    }

    fn does_not_contain_element<'a, T, I>(things: I, element: &Element) -> bool
        where T: WithElement + 'a,
              I: IntoIterator<Item = &'a T>
    {
        things.into_iter().all(|t| t.element_name() != element.0)
    }

    fn contains_generator_type(&self, element: &Element) -> bool {
        FloorContents::contains_element(&self.generators, element)
    }

    fn contains_generator_not_of_type(&self, element: &Element) -> bool {
        FloorContents::does_not_contain_element(&self.generators, element)
    }

    fn contains_microchip_type(&self, element: &Element) -> bool {
        FloorContents::contains_element(&self.microchips, element)
    }

    fn contains_microchip_not_of_type(&self, element: &Element) -> bool {
        FloorContents::does_not_contain_element(&self.microchips, element)
    }

    fn can_accept_microchip(&self, microchip: &Microchip) -> bool {
        !self.contains_generator_not_of_type(&microchip.0)
    }

    fn can_accept_generator(&self, generator: &Generator) -> bool {
        !self.contains_microchip_not_of_type(&generator.0)
    }

    fn is_valid_without_microchip(&self, microchip: &Microchip) -> bool {
        true
    }

    fn contains_unmatched_generator_not_of_type(&self, element: &Element) -> bool {
        self.generators
            .iter()
            .filter(|g| g.element_name() != element.0)
            .filter(|g| !self.contains_microchip_type(&g.0))
            .next()
            .is_some() // check that it's non-empty
    }

    fn is_valid_without_generator(&self, generator: &Generator) -> bool {
        // valid if tne floor has a microchip of the given type and no unmatched generators of other types to fry it
        // or if the floor doesn't contain a microchip of the given type
        (self.contains_microchip_type(&generator.0) &&
         !self.contains_unmatched_generator_not_of_type(&generator.0)) ||
        !self.contains_microchip_type(&generator.0)
    }

    fn can_accept_generators(&self, generator1: &Generator, generator2: &Generator) -> bool {
        self.can_accept_generator(generator1) && self.can_accept_generator(generator2)
    }

    fn can_accept_microchips(&self, microchip1: &Microchip, microchip2: &Microchip) -> bool {
        self.can_accept_microchip(microchip1) && self.can_accept_microchip(microchip2)
    }

    fn can_accept_generator_and_microchip(&self,
                                          generator: &Generator,
                                          microchip: &Microchip)
                                          -> bool {
        self.can_accept_microchip(microchip) && self.can_accept_generator(generator)
    }

    fn is_valid_without_generators(&self, generator1: &Generator, generator2: &Generator) -> bool {
        self.is_valid_without_generator(generator1) && self.is_valid_without_generator(generator2)
    }

    fn is_valid_without_microchips(&self, microchip1: &Microchip, microchip2: &Microchip) -> bool {
        self.is_valid_without_microchip(microchip1) && self.is_valid_without_microchip(microchip2)
    }

    fn is_valid_without_generator_and_microchip(&self,
                                                generator: &Generator,
                                                microchip: &Microchip)
                                                -> bool {
        self.is_valid_without_generator(generator) && self.is_valid_without_microchip(microchip)
    }
}

impl WithElement for Generator {
    fn element_name(&self) -> String {
        (self.0).clone().0
    }
}

impl WithElement for Microchip {
    fn element_name(&self) -> String {
        (self.0).clone().0
    }
}

impl Compatible<Microchip> for Generator {
    fn is_compatible_with(&self, other: &Microchip) -> bool {
        (self.0).0 == (other.0).0
    }
}

impl Compatible<Generator> for Microchip {
    fn is_compatible_with(&self, other: &Generator) -> bool {
        (self.0).0 == (other.0).0
    }
}

impl fmt::Display for Generator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} generator", (self.0).0)
    }
}

impl fmt::Display for Microchip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-compatible microchip", (self.0).0)
    }
}

#[test]
fn test_compatibility() {
    let hchip = Microchip(Element("hydrogen".to_owned()));
    let hgen = Generator(Element("hydrogen".to_owned()));
    let rchip = Microchip(Element("rubidium".to_owned()));
    let rgen = Generator(Element("rubidium".to_owned()));

    assert!(hchip.is_compatible_with(&hgen));
    assert!(hgen.is_compatible_with(&hchip));
    assert!(rchip.is_compatible_with(&rgen));
    assert!(rgen.is_compatible_with(&rchip));

    assert!(!hchip.is_compatible_with(&rgen));
    assert!(!hgen.is_compatible_with(&rchip));
    assert!(!rchip.is_compatible_with(&hgen));
    assert!(!rgen.is_compatible_with(&hchip));
}
