extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;
extern crate md5;
extern crate rustc_serialize;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate nom_test_helpers;

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
// mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
// mod day22;
mod day23;

mod assembunny;

mod nom_helpers;

use std::env;

fn main() {
    let first_arg = env::args()
        .nth(1)
        .expect("Expected command line argument to tell me which day's solution to run.");
    let desired_daynum = first_arg.parse::<u32>()
        .expect("Unable to parse the provided argument - is it a number?");

    match desired_daynum {
        1 => {
            print_day_header(1, false);
            day1::do_day1();
        }
        2 => {
            print_day_header(2, false);
            let input = include_str!("inputs/day2_input.txt");
            day2::do_day2(input);
        }
        3 => {
            print_day_header(3, false);

            day3::do_day3();
        }
        4 => {
            print_day_header(4, false);

            day4::do_day4();
        }
        5 => {
            print_day_header(5, true);

            day5::do_day5();
        }
        6 => {
            print_day_header(6, false);
            let input = include_str!("inputs/day6_input.txt");

            day6::do_day6(input);
        }
        7 => {
            print_day_header(7, false);
            let input = include_str!("inputs/day7_input.txt");

            day7::do_day7(input);
        }
        8 => {
            print_day_header(8, false);
            let input = include_str!("inputs/day8_input.txt");

            day8::do_day8(input);
        }
        9 => {
            print_day_header(9, false);
            let input = include_str!("inputs/day9_input.txt");

            day9::do_day9(input);
        }
        10 => {
            print_day_header(10, false);
            let input = include_str!("inputs/day10_input.txt");

            day10::do_day10(input);
        }
        11 => {
            print_day_header(11, true);
            // let input = include_str!("inputs/day11_input.txt");

            // day11::do_day11(input);
            println!("Day 11 incomplete");
        }
        12 => {
            print_day_header(12, false);
            let input = include_str!("inputs/day12_input.txt");

            day12::do_day12(input);
        }
        13 => {
            print_day_header(13, false);
            day13::do_day13();
        }
        14 => {
            print_day_header(14, true);
            day14::do_day14();
        }
        15 => {
            print_day_header(15, false);
            day15::do_day15();
        }
        16 => {
            print_day_header(16, false);
            day16::do_day16();
        }
        17 => {
            print_day_header(17, true);
            day17::do_day17();
        }
        18 => {
            print_day_header(18, false);
            day18::do_day18();
        }
        19 => {
            print_day_header(19, false);
            day19::do_day19();
        }
        20 => {
            print_day_header(20, false);
            let input = include_str!("inputs/day20.txt");
            day20::do_day20(input);
        }
        21 => {
            print_day_header(21, true);
            let input = include_str!("inputs/day21.txt");
            day21::do_day21(input);
        }
        22 => {
            print_day_header(22, false);
            // let input = include_str!("inputs/day22.txt");
            // day22::do_day22(input);
        }
        23 => {
            print_day_header(23, true);
            let input = include_str!("inputs/day23.txt");
            day23::do_day23(input);
        }
        _ => println!("I'm sorry, I can't handle day {} yet", desired_daynum),
    }
}

fn print_day_header(day: u32, is_slow: bool) {
    println!("Day {} coming right up...", day);
    if is_slow {
        println!("This one might take a while though");
    }
    println!("");
}
