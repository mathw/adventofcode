# Advent of Code 2016

These are my solutions for Advent of Code 2016. They are all written in Rust 1.13.

Although my solution structure varied a lot from day to day initially, I have since organised it all.

To run any given day's solution, just get your Rust-enabled command line and type:

    cargo run 3

And you'll get day 3. Extrapolate from this for other days and you'll probably be right.

## I'm not very good at Rust

That's why I'm doing this in the first place!

I'm getting better though. At the time of writing, some of my early solutions have been restructured and/or improved based on things I've learned later, so if you want to see how terrible it really was at the start go back in the commit history...

## Requirements

To build all you need is Rust 1.13 and Cargo. An easy way to get that is to get Rustup from https://rustup.rs and install stable Rust. Provided it's a 1.x version where x >= 13 it should happily build and run everything after fetching the various third-party libraries required. Rustup is beta at time of writing, but it's working well for me so far. Alternatively, if you've got distro/OS packages or an installer for Rust >= 1.13 then go for that route. The Rust installers give you Cargo as well.

Some of it might run with earlier versions of Rust. I have yet to consciously use a 1.13 feature, but I have no idea whether I'm using 1.12 features or not at this point.
