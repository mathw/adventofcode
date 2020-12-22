use crate::dayerror::DayError;
use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    hash::{Hash, Hasher},
    str::FromStr,
};

pub fn part1() -> Result<String, DayError> {
    let score = run_part1(include_str!("player1.txt"), include_str!("player2.txt"))?;
    Ok(format!("The winning player's score is {}", score))
}

fn parse_deck(s: &str) -> Result<VecDeque<u8>, DayError> {
    let v = s
        .lines()
        .map(|x| u8::from_str(x))
        .collect::<Result<Vec<_>, _>>()?;
    // a shame there's no FromIterator<Result<x,y>> for Result<VecDeque<x>,y> like there is for Vec
    Ok(v.into())
}

fn play_round(player1: &mut VecDeque<u8>, player2: &mut VecDeque<u8>) -> Result<(), DayError> {
    let card1 = player1
        .pop_front()
        .ok_or_else(|| DayError::NoSolutionFoundError)?;
    let card2 = player2
        .pop_front()
        .ok_or_else(|| DayError::NoSolutionFoundError)?;

    if card1 > card2 {
        player1.push_back(card1);
        player1.push_back(card2);
    } else if card2 > card1 {
        player2.push_back(card2);
        player2.push_back(card1);
    } else {
        return Err(DayError::NoSolutionFoundError);
    }
    Ok(())
}

fn play_game(player1: &mut VecDeque<u8>, player2: &mut VecDeque<u8>) -> Result<(), DayError> {
    while !player1.is_empty() && !player2.is_empty() {
        play_round(player1, player2)?
    }
    Ok(())
}

fn calculate_score(winning_deck: &VecDeque<u8>) -> u32 {
    winning_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i as u32 + 1) * *c as u32)
        .sum()
}

fn run_part1(input1: &str, input2: &str) -> Result<u32, DayError> {
    let mut player1 = parse_deck(input1)?;
    let mut player2 = parse_deck(input2)?;

    play_game(&mut player1, &mut player2)?;

    let score = if player1.is_empty() {
        calculate_score(&player2)
    } else {
        calculate_score(&player1)
    };
    Ok(score)
}

#[derive(Eq, PartialEq, Hash)]
struct GameState {
    player1: Vec<u8>,
    player2: Vec<u8>,
}

impl GameState {
    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.player1.hash(&mut hasher);
        self.player2.hash(&mut hasher);
        hasher.finish()
    }
}

impl From<(VecDeque<u8>, VecDeque<u8>)> for GameState {
    fn from(decks: (VecDeque<u8>, VecDeque<u8>)) -> Self {
        GameState {
            player1: decks.0.into(),
            player2: decks.1.into(),
        }
    }
}

fn play_recursive_game(
    player1: &VecDeque<u8>,
    player2: &VecDeque<u8>,
    visualise: bool,
) -> Result<u32, DayError> {
    let winner = play_recursive_subgame(player1.clone(), player2.clone(), 0, visualise)?;

    match winner {
        Winner::Player1(score) => Ok(score),
        Winner::Player2(score) => Ok(score),
    }
}

#[derive(PartialEq, Eq)]
enum Winner {
    Player1(u32),
    Player2(u32),
}

fn play_recursive_subgame(
    mut player1: VecDeque<u8>,
    mut player2: VecDeque<u8>,
    depth: usize,
    visualise: bool,
) -> Result<Winner, DayError> {
    let depth_str = vec!["  "; depth].into_iter().collect::<String>();
    let mut state_memory = HashSet::new();
    while !player1.is_empty() && !player2.is_empty() {
        if visualise {
            println!("{}Player 1: {:?}", depth_str, player1);
            println!("{}Player 2: {:?}", depth_str, player2);
        }
        if !state_memory.insert(GameState::from((player1.clone(), player2.clone())).hash()) {
            if visualise {
                println!("{}Game state duplicated - player 1 wins", depth_str);
            }
            // infinite game prevention rule triggered
            return Ok(Winner::Player1(calculate_score(&player1)));
        }
        if visualise {
            println!(
                "{}Recorded {} unique game states",
                depth_str,
                state_memory.len()
            );
        }

        let card1 = player1.pop_front().ok_or(DayError::NoSolutionFoundError)?;
        let card2 = player2.pop_front().ok_or(DayError::NoSolutionFoundError)?;

        if visualise {
            println!(
                "{}Player 1 drew {}, player 2 drew {}",
                depth_str, card1, card2
            );
        }

        if card1 as usize <= player1.len() && card2 as usize <= player2.len() {
            if visualise {
                println!(
                    "{}Triggering recursive subgame to decide this round",
                    depth_str
                );
            }
            // decide this round by playing a recursive subgame
            match play_recursive_subgame(
                player1.iter().take(card1 as usize).cloned().collect(),
                player2.iter().take(card2 as usize).cloned().collect(),
                depth + 1,
                visualise,
            )? {
                Winner::Player1(_) => {
                    player1.push_back(card1);
                    player1.push_back(card2);
                }
                Winner::Player2(_) => {
                    player2.push_back(card2);
                    player2.push_back(card1);
                }
            }
        } else {
            if card1 > card2 {
                if visualise {
                    println!("{}Player 1 wins the round", depth_str);
                }
                player1.push_back(card1);
                player1.push_back(card2);
            } else {
                if visualise {
                    println!("{}Player 2 wins the round", depth_str);
                }
                player2.push_back(card2);
                player2.push_back(card1);
            }
        }
    }
    Ok(if player1.is_empty() {
        if visualise {
            println!("{}Player 2 wins the sub-game", depth_str);
        }
        Winner::Player2(calculate_score(&player2))
    } else {
        if visualise {
            println!("{}Player 1 wins the sub-game", depth_str);
        }
        Winner::Player1(calculate_score(&player1))
    })
}

fn run_part2(player1: &str, player2: &str, visualise: bool) -> Result<u32, DayError> {
    let player1 = parse_deck(player1)?;
    let player2 = parse_deck(player2)?;

    play_recursive_game(&player1, &player2, visualise)
}

pub fn part2(visualise: bool) -> Result<String, DayError> {
    let score = run_part2(
        include_str!("player1.txt"),
        include_str!("player2.txt"),
        visualise,
    )?;

    Ok(format!("The winning score is {}", score))
}

#[test]
fn test_recursive_combat() {
    let result = run_part2(
        "9
2
6
3
1",
        "5
8
4
7
10",
        true,
    )
    .unwrap();
    assert_eq!(result, 291);
}

#[test]
fn test_state() {
    let mut state = HashSet::new();
    let state1 = GameState::from((VecDeque::new(), VecDeque::new()));
    let state2 = GameState::from((VecDeque::new(), VecDeque::new()));
    state.insert(state1);
    assert!(!state.insert(state2));
}
