# Advent of Code 2016

These are my solutions for Advent of Code 2016. They are all written in Rust 1.13.

I have wavered about on how to structure the code from day to day. Therefore, there aren't neat little binaries for each day to build and run - it's one giant codebase but main.rs changes each day to import and run only what it needs to.

The best way to actually run each solution if you should for some bizarre reason want to do that is to check out the commit for that day. Some days have a commit for part one and part two, while later days mostly have just one commit per day and deliver both answers with a single run.

## Development of Rust skill

This is something you may notice quite substantially over the first few days in particular. Also, each time I use a new complex library (particularly thinking about my use of Nom in day 9 and then in day 10) it gets better-used if it's used again a second time.

Rust is an awesome language, but the docs don't always deliver when you need them to and like all ecosystems the third-party library documentation is very variable indeed.
