#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Bot {
    holds1: Option<u32>,
    holds2: Option<u32>,
}

impl Bot {
    pub fn new() -> Bot {
        Bot {
            holds1: None,
            holds2: None,
        }
    }

    pub fn is_holding(&self, what: u32) -> bool {
        self.holds1.map(|h| h == what).unwrap_or(false) ||
        self.holds2.map(|h| h == what).unwrap_or(false)
    }

    pub fn holds_two(&self) -> bool {
        self.holds1.is_some() && self.holds2.is_some()
    }

    pub fn get_lower(&self) -> Option<u32> {
        if !self.holds_two() {
            None
        } else {
            let (one, two) = (self.holds1.unwrap(), self.holds2.unwrap());
            if one < two { Some(one) } else { Some(two) }
        }
    }

    pub fn get_higher(&self) -> Option<u32> {
        if !self.holds_two() {
            None
        } else {
            let (one, two) = (self.holds1.unwrap(), self.holds2.unwrap());
            if one > two { Some(one) } else { Some(two) }
        }
    }

    pub fn take_value(&mut self, what: u32) -> bool {
        if self.holds1.is_some() && self.holds1.unwrap() == what {
            self.holds1 = None;
            return true;
        } else if self.holds2.is_some() && self.holds2.unwrap() == what {
            self.holds2 = None;
            return true;
        }

        return false;
    }

    pub fn give_value(&mut self, what: u32) -> bool {
        if self.holds1.is_none() {
            self.holds1 = Some(what);
            return true;
        }
        if self.holds2.is_none() {
            self.holds2 = Some(what);
            return true;
        }
        return false;
    }
}


#[test]
fn test_bot_is_holding() {
    let bot = Bot::new_holding(5);
    let bot2 = Bot::new_holding2(5, 6);

    assert!(bot.is_holding(5));
    assert!(!bot.is_holding(2));
    assert!(bot2.is_holding(6));
    assert!(bot2.is_holding(5));
    assert!(!bot2.is_holding(23));
}

#[test]
fn test_give_take() {
    let mut bot = Bot::new();

    bot.give_value(2);
    assert!(bot.is_holding(2));
    assert!(!bot.is_holding(3));
    bot.take_value(2);
    assert!(!bot.is_holding(2));
    bot.give_value(2);
    bot.give_value(4);
    assert!(bot.is_holding(2));
    assert!(!bot.is_holding(3));
    assert!(bot.is_holding(4));
    bot.take_value(4);
    assert!(bot.is_holding(2));
    assert!(!bot.is_holding(3));
    assert!(!bot.is_holding(4));
}
