/// run a state machine - the only public function here!
pub fn run(input: &str) -> (u32, u32) {
    #[cfg(test)]
    println!("\ninput: {}", input);

    let mut state = State::base();
    for event in make_event_stream(input) {
        #[cfg(test)]
        println!("{:?} {:?}", state, event);

        state = transition(state, event);
    }
    (state.score(), state.garbage())
}

fn make_event_stream(input: &str) -> Vec<Event> {
    let mut escaping = false;
    let mut result = Vec::new();

    for current in input.chars() {
        if escaping {
            // this character is ignored
            escaping = false;
            continue;
        }
        if current == '!' {
            escaping = true;
            continue;
        }

        result.push(current.into());
    }

    result
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Base(StateS<Base>),
    InGroup(StateS<Group>),
    InGarbage(StateS<Garbage>),
}

impl State {
    /// Create a new base state
    fn base() -> State {
        State::Base(StateS {
            score: 0,
            garbage: 0,
            state: Base {},
        })
    }

    /// Return a state's score
    fn score(&self) -> u32 {
        match self {
            &State::InGroup(StateS { score: s, garbage: _, state: _ }) => s,
            &State::InGarbage(StateS { score: s, garbage: _, state: _ }) => s,
            &State::Base(StateS { score: s, garbage: _, state: _ }) => s,
        }
    }

    /// Return a state's garbage count
    fn garbage(&self) -> u32 {
        match self {
            &State::InGroup(StateS { score: _, garbage: g, state: _ }) => g,
            &State::InGarbage(StateS { score: _, garbage: g, state: _ }) => g,
            &State::Base(StateS { score: _, garbage: g, state: _ }) => g,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct StateS<S>
    where S: Copy
{
    score: u32,
    garbage: u32,
    state: S,
}

impl<S> StateS<S>
    where S: Copy
{
    fn move_to<T>(self, new_state: T) -> StateS<T>
        where T: Copy
    {
        StateS {
            score: self.score,
            garbage: self.garbage,
            state: new_state,
        }
    }

    fn move_increase_score_by<T>(self, score_diff: u32, new_state: T) -> StateS<T>
        where T: Copy
    {
        StateS {
            score: self.score + score_diff,
            garbage: self.garbage,
            state: new_state,
        }
    }

    fn increase_garbage(self) -> StateS<S> {
        StateS {
            score: self.score,
            garbage: self.garbage + 1,
            state: self.state,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Base {
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Garbage {
    group_level: Option<u32>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Group {
    group_level: u32,
}

impl From<StateS<Base>> for StateS<Garbage> {
    fn from(base: StateS<Base>) -> StateS<Garbage> {
        base.move_to(Garbage { group_level: None })
    }
}

impl From<StateS<Group>> for StateS<Garbage> {
    fn from(base: StateS<Group>) -> StateS<Garbage> {
        base.move_to(Garbage { group_level: Some(base.state.group_level) })
    }
}

impl From<StateS<Base>> for StateS<Group> {
    fn from(base: StateS<Base>) -> StateS<Group> {
        base.move_to(Group { group_level: 0 })
    }
}

impl StateS<Group> {
    fn deeper(self) -> StateS<Group> {
        self.move_to(Group { group_level: self.state.group_level + 1 })
    }

    fn shallower(self) -> StateS<Group> {
        self.move_increase_score_by(self.state.group_level + 1,
                                    Group { group_level: self.state.group_level - 1 })
    }
}

impl From<StateS<Group>> for StateS<Base> {
    fn from(group: StateS<Group>) -> StateS<Base> {
        group.move_increase_score_by(1, Base {})
    }
}

impl From<StateS<Garbage>> for StateS<Base> {
    fn from(garbage: StateS<Garbage>) -> StateS<Base> {
        garbage.move_to(Base {})
    }
}

impl From<StateS<Garbage>> for StateS<Group> {
    fn from(garbage: StateS<Garbage>) -> StateS<Group> {
        garbage.move_to(Group { group_level: garbage.state.group_level.unwrap() })
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Event {
    OpenGroup,
    CloseGroup,
    OpenGarbage,
    CloseGarbage,
    NoEvent,
}

impl From<char> for Event {
    fn from(c: char) -> Event {
        match c {
            '{' => Event::OpenGroup,
            '}' => Event::CloseGroup,
            '<' => Event::OpenGarbage,
            '>' => Event::CloseGarbage,
            _ => Event::NoEvent,
        }
    }
}

/// Consume a state to react to an event.
/// Always succeeds. If the event is invalid the same state is returned.
fn transition(state: State, event: Event) -> State {
    match (&state, event) {
        (&State::Base(base), Event::OpenGroup) => State::InGroup(base.into()),
        (&State::Base(base), Event::OpenGarbage) => State::InGarbage(base.into()),
        (&State::Base(_), Event::CloseGroup) => state,
        (&State::Base(_), Event::CloseGarbage) => state,
        (&State::InGroup(group), Event::OpenGroup) => State::InGroup(group.deeper()),
        (&State::InGroup(s @ StateS { state: Group { group_level: 0 }, .. }),
         Event::CloseGroup) => State::Base(s.into()),
        (&State::InGroup(s @ StateS { state: Group { group_level: _ }, .. }),
         Event::CloseGroup) => State::InGroup(s.shallower()),
        (&State::InGroup(group), Event::OpenGarbage) => State::InGarbage(group.into()),
        (&State::InGroup(_), Event::CloseGarbage) => state,
        (&State::InGarbage(s @ StateS { state: Garbage { group_level: None }, .. }),
         Event::CloseGarbage) => State::Base(s.into()),
        (&State::InGarbage(s @ StateS { state: Garbage { group_level: Some(_) }, .. }),
         Event::CloseGarbage) => State::InGroup(s.into()),
        (&State::InGarbage(s), _) => State::InGarbage(s.increase_garbage()),
        (_, Event::NoEvent) => state,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_close() {
        let state = State::base();
        let state = transition(state, Event::OpenGroup);
        let state = transition(state, Event::CloseGroup);

        assert_eq!(state.score(), 1);
    }

    #[test]
    fn open_open_close_close() {
        let state = State::base();
        let state = transition(state, Event::OpenGroup);
        let state = transition(state, Event::OpenGroup);
        let state = transition(state, Event::CloseGroup);
        let state = transition(state, Event::CloseGroup);

        assert_eq!(state.score(), 3);
    }

    #[test]
    fn open_close_open_close() {
        let state = State::base();
        let state = transition(state, Event::OpenGroup);
        let state = transition(state, Event::CloseGroup);
        let state = transition(state, Event::OpenGroup);
        let state = transition(state, Event::CloseGroup);

        assert_eq!(state.score(), 2);
    }

    #[test]
    fn open_garbage_close_garbage_closegarbage_close() {
        let state = State::base();
        println!("{:?}", state);
        let state = transition(state, Event::OpenGroup);
        println!("{:?}", state);
        let state = transition(state, Event::OpenGarbage);
        println!("{:?}", state);
        let state = transition(state, Event::CloseGroup);
        println!("{:?}", state);
        let state = transition(state, Event::OpenGarbage);
        println!("{:?}", state);
        let state = transition(state, Event::CloseGarbage);
        println!("{:?}", state);
        let state = transition(state, Event::CloseGroup);
        println!("{:?}", state);

        assert_eq!(state.score(), 1);
    }

    #[test]
    fn run_aoc_examples() {
        assert_eq!(run("{}"), (1, 0));
        assert_eq!(run("{{{}}}"), (6, 0));
        assert_eq!(run("{{},{}}"), (5, 0));
        assert_eq!(run("{{{},{},{{}}}}"), (16, 0));
        assert_eq!(run("{<a>,<a>,<a>,<a>}"), (1, 4));
        assert_eq!(run("{{<ab>},{<ab>},{<ab>},{<ab>}}"), (9, 8));
        assert_eq!(run("{{<!!>},{<!!>},{<!!>},{<!!>}}"), (9, 0));
        assert_eq!(run("{{<a!>},{<a!>},{<a!>},{<ab>}}"), (3, 17));
    }

    #[test]
    fn run_part2_aoc_examples() {
        assert_eq!(run("<>").1, 0);
        assert_eq!(run("<random characters>").1, 17);
        assert_eq!(run("<<<<>").1, 3);
        assert_eq!(run("<{!>}>").1, 2);
        assert_eq!(run("<!!>").1, 0);
        assert_eq!(run("<!!!>>").1, 0);
        assert_eq!(run("<{o\"i!a,<{i<a>").1, 10);
    }
}