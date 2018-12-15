mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod util;

use crate::day::Day;
use crate::util::timed;
use crate::util::ErrString;
use std::env::args;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() -> Result<(), String> {
    println!("Advent of Code 2018");

    let args = args().collect::<Vec<_>>();

    if args.len() < 2 {
        Err("You must supply a day number to run a puzzle".into())
    } else {
        let day = usize::from_str(&args[1]).err_string()?;
        println!("You have requested day {}", day);

        let (result, time) = match day {
            1 => timed(|| run_day(Arc::new(Mutex::new(day1::Day1::new())))),
            2 => timed(|| run_day(Arc::new(Mutex::new(day2::Day2::new())))),
            3 => timed(|| run_day(Arc::new(Mutex::new(day3::Day3::new())))),
            4 => timed(|| {
                run_day(Arc::new(Mutex::new(
                    day4::Day4::new().expect("Day 4 could not parse input"),
                )))
            }),
            5 => timed(|| {
                run_day(Arc::new(Mutex::new(
                    day5::Day5::new().expect("Day 5 could not parse input"),
                )))
            }),
            6 => timed(|| run_day(Arc::new(Mutex::new(day6::Day6::new())))),
            7 => timed(|| {
                run_day(Arc::new(Mutex::new(
                    day7::Day7::new().expect("Day 7 could not parse input"),
                )))
            }),
            8 => timed(|| run_day(Arc::new(Mutex::new(day8::Day8::new())))),
            9 => timed(|| run_day(Arc::new(Mutex::new(day9::Day9::new())))),
            _ => (Err(format!("I don't know how to be day {} yet", day)), 0),
        };

        println!("Total time: {}ms", time);
        result
    }
}

fn run_day<D: 'static + Day + Send>(day: Arc<Mutex<D>>) -> Result<(), String> {
    let (sender1, receiver1) = channel();
    let (sender2, receiver2) = channel();

    let receive_thread1 = thread::spawn(move || loop {
        let received = receiver1.recv();
        match received {
            Ok(msg) => println!("[1] {}", msg),
            Err(_) => return,
        }
    });
    let receive_thread2 = thread::spawn(move || loop {
        let received = receiver2.recv();
        match received {
            Ok(msg) => println!("[2] {}", msg),
            Err(_) => return,
        }
    });

    let day_a = day.clone();
    let part1 = thread::spawn(move || {
        let (_, time) = timed(|| day_a.lock().unwrap().part1(&sender1));
        sender1.send(format!("Part one: {}ms", time)).unwrap();
    });

    part1.join().unwrap();
    receive_thread1.join().unwrap();

    let part2 = thread::spawn(move || {
        let (_, time) = timed(|| day.lock().unwrap().part2(&sender2));
        sender2.send(format!("Part two: {}ms", time)).unwrap();
    });
    part2.join().unwrap();
    receive_thread2.join().unwrap();

    Ok(())
}
