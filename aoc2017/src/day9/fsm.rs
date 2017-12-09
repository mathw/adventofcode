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

#[derive(Debug, PartialEq, Clone)]
enum State {
    InGroup {
        group_level: u32,
        score: u32,
        garbage: u32,
    },
    InGarbage {
        group_level: Option<u32>,
        score: u32,
        garbage: u32,
    },
    Base { score: u32, garbage: u32 },
}

impl State {
    /// Create a new base state
    fn base() -> State {
        State::Base {
            score: 0,
            garbage: 0,
        }
    }

    /// Return a state's score
    fn score(&self) -> u32 {
        match self {
            &State::InGroup { group_level: _, score: s, garbage: _ } => s,
            &State::InGarbage { group_level: _, score: s, garbage: _ } => s,
            &State::Base { score: s, garbage: _ } => s,
        }
    }

    /// Return a state's garbage count
    fn garbage(&self) -> u32 {
        match self {
            &State::InGroup { group_level: _, score: _, garbage: g } => g,
            &State::InGarbage { group_level: _, score: _, garbage: g } => g,
            &State::Base { score: _, garbage: g } => g,
        }
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
        (&State::Base { score: s, garbage: g }, Event::OpenGroup) => {
            State::InGroup {
                group_level: 0,
                score: s,
                garbage: g,
            }
        }
        (&State::Base { score: s, garbage: g }, Event::OpenGarbage) => {
            State::InGarbage {
                group_level: None,
                score: s,
                garbage: g,
            }
        }
        (&State::Base { score: _, garbage: _ }, Event::CloseGroup) => state,
        (&State::Base { score: _, garbage: _ }, Event::CloseGarbage) => state,
        (&State::InGroup { group_level: gl, score: s, garbage: g }, Event::OpenGroup) => {
            State::InGroup {
                group_level: gl + 1,
                score: s,
                garbage: g,
            }
        }
        (&State::InGroup { group_level: 0, score: s, garbage: g }, Event::CloseGroup) => {
            State::Base {
                score: s + 1,
                garbage: g,
            }
        }
        (&State::InGroup { group_level: gl, score: s, garbage: g }, Event::CloseGroup) => {
            State::InGroup {
                group_level: gl - 1,
                score: s + 1 + gl,
                garbage: g,
            }
        }
        (&State::InGroup { group_level: gl, score: s, garbage: g }, Event::OpenGarbage) => {
            State::InGarbage {
                group_level: Some(gl),
                score: s,
                garbage: g,
            }
        }
        (&State::InGroup { group_level: _, score: _, garbage: _ }, Event::CloseGarbage) => state,
        (&State::InGarbage { group_level: None, score: s, garbage: g }, Event::CloseGarbage) => {
            State::Base {
                score: s,
                garbage: g,
            }
        }
        (&State::InGarbage { group_level: Some(gl), score: s, garbage: g },
         Event::CloseGarbage) => {
            State::InGroup {
                group_level: gl,
                score: s,
                garbage: g,
            }
        }
        (&State::InGarbage { group_level: gl, score: s, garbage: g }, _) => {
            State::InGarbage {
                group_level: gl,
                score: s,
                garbage: g + 1,
            }
        }
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