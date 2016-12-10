use super::bot::Bot;
use super::instructions::{Target, Instruction};
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Debug)]
pub struct State {
    bots: BTreeMap<u32, Bot>,
    outputs: BTreeMap<u32, Vec<u32>>,
    decisions: BTreeMap<u32, (Target, Target)>,
}

impl State {
    pub fn new() -> State {
        State {
            bots: BTreeMap::new(),
            outputs: BTreeMap::new(),
            decisions: BTreeMap::new(),
        }
    }

    pub fn act_on(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Give { value, to } => self.give_value_to_bot(value, to),
            Instruction::Decide { bot, low, high } => self.add_decision(bot, low, high),
        }
    }

    /// run instructions until there is a bot holding the two specified values
    pub fn run_until_bot_holds(&mut self, values: (u32, u32)) -> Option<u32> {
        for (botnum, (low_to, high_to)) in self.decisions.clone() {
            // first check to see if a bot is now holding those values
            let botholding = self.find_bot_holding(values.0, values.1);
            if botholding.is_some() {
                return botholding;
            }

            // a bot is not holding those values. Evaluate a decision.
            self.run_decision(botnum, low_to, high_to);
        }

        // perhaps we have to run all the decisions again?
        self.run_until_bot_holds(values)
    }

    /// Run the decisions until no changes happen
    pub fn run_all_decisions(&mut self) {
        loop {
            let mut changed_this_loop = false;
            for (botnum, (low_to, high_to)) in self.decisions.clone() {
                let changed = self.run_decision(botnum, low_to, high_to);
                changed_this_loop = changed_this_loop || changed;
            }

            if !changed_this_loop {
                // we have run out of things to do
                return;
            }
        }
    }

    pub fn get_output_bin(&self, bin: u32) -> Vec<u32> {
        self.outputs.get(&bin).unwrap_or(&Vec::new()).clone()
    }

    fn give_value_to_bot(&mut self, value: u32, bot_number: u32) {
        let mut bot = self.bots.entry(bot_number).or_insert(Bot::new());
        let result = bot.give_value(value);
        if !result {
            panic!("Unable to give value to bot");
        }
    }

    fn put_value_in_output(&mut self, value: u32, output: u32) {
        let mut output = self.outputs.entry(output).or_insert(Vec::new());
        output.push(value);
    }

    fn add_decision(&mut self, bot_number: u32, low_to: Target, high_to: Target) {
        self.decisions.insert(bot_number, (low_to, high_to));
    }

    fn run_decision(&mut self, bot_number: u32, low_to: Target, high_to: Target) -> bool {
        let lower;
        let higher;

        {
            let mut bot = self.bots.entry(bot_number).or_insert(Bot::new());

            if !bot.holds_two() {
                return false;
            }

            lower = bot.get_lower().unwrap();
            higher = bot.get_higher().unwrap();

            bot.take_value(lower);
            bot.take_value(higher);
        }

        self.give_value_to(lower, low_to);
        self.give_value_to(higher, high_to);

        true
    }

    fn give_value_to(&mut self, value: u32, to: Target) {
        match to {
            Target::Bot(bot) => self.give_value_to_bot(value, bot),
            Target::Output(output) => self.put_value_in_output(value, output),
        }
    }

    fn find_bot_holding(&self, value1: u32, value2: u32) -> Option<u32> {
        self.bots
            .iter()
            .filter(|&(_, bot)| bot.is_holding(value1) && bot.is_holding(value2))
            .next()
            .map(|(botnum, _)| *botnum)
    }
}


#[test]
fn test_find_bot_holding() {
    let mut state = State::new();
    let val1 = 3;
    let val2 = 6;
    let botnum = 8;

    state.give_value_to_bot(val1, botnum);
    state.give_value_to_bot(val2, botnum);

    let optbot = state.find_bot_holding(val1, val2);
    assert_eq!(optbot, Some(botnum));
}

#[test]
fn test_find_bot_holding_not_holding() {
    let mut state = State::new();
    let val1 = 3;
    let val2 = 6;
    let botnum = 8;

    state.give_value_to_bot(val1, botnum);

    let optbot = state.find_bot_holding(val1, val2);
    assert_eq!(optbot, None);
}
