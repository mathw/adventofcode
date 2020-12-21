pub(crate) mod expressions;
lalrpop_mod!(grammar, "/day18/grammar.rs");
lalrpop_mod!(grammar2, "/day18/grammar2.rs");

use crate::dayerror::DayError;
use expressions::{Expr, Expression, Factor, Operand, Term};

pub fn part1() -> Result<String, DayError> {
    let result = run_part1(include_str!("input.txt"))?;

    Ok(format!("The sum of all expressions is {}", result))
}

pub fn part2() -> Result<String, DayError> {
    let result = run_part2(include_str!("input.txt"))?;

    Ok(format!("The sum of all expressions is {}", result))
}

fn run_part1(input: &str) -> Result<u64, DayError> {
    let parser = grammar::ExprParser::new();
    let mut sum = 0;

    for line in input.lines() {
        let expression = parser.parse(line).map_err(|_| {
            DayError::InputParseError(format!(
                "Unhelpful error parsing the input line \"{}\"",
                line
            ))
        })?;
        let result = evaluate(&expression);
        sum += result;
    }

    Ok(sum)
}

fn run_part2(input: &str) -> Result<u64, DayError> {
    let parser = grammar2::ExprParser::new();
    let mut sum = 0;

    for line in input.lines() {
        let expression = parser.parse(line).map_err(|_| {
            DayError::InputParseError(format!(
                "Unhelpful error parsing the input line \"{}\"",
                line
            ))
        })?;
        let result = evaluate2(&expression);
        sum += result;
    }

    Ok(sum)
}

fn evaluate(e: &Expression) -> u64 {
    match e {
        Expression::Operand(o) => evaluate_operand(o),
        Expression::Add { left, right } => evaluate(left) + evaluate_operand(right),
        Expression::Multiply { left, right } => evaluate(left) * evaluate_operand(right),
    }
}

fn evaluate_operand(o: &Operand) -> u64 {
    match o {
        Operand::Number(n) => (*n) as u64,
        Operand::Expression(e) => evaluate(e),
    }
}

fn evaluate2(e: &Expr) -> u64 {
    match e {
        Expr::Multiply(l, r) => evaluate2(l) * evaluate2_factor(r),
        Expr::Factor(p) => evaluate2_factor(p),
    }
}

fn evaluate2_factor(p: &Factor) -> u64 {
    match p {
        Factor::Add(l, r) => evaluate2_factor(l) + evaluate2_term(r),
        Factor::Term(o) => evaluate2_term(o),
    }
}

fn evaluate2_term(t: &Term) -> u64 {
    match t {
        Term::Number(n) => *n,
        Term::Expr(e) => evaluate2(&e),
    }
}

#[test]
fn test_grammar() {
    let parser = grammar::ExprParser::new();
    let result = parser.parse("22 + 4").unwrap();
    assert_eq!(
        result,
        Expression::Add {
            left: Box::new(Expression::Operand(Operand::Number(22))),
            right: Operand::Number(4)
        }
    );

    let result = parser.parse("22 + 4 * 9").unwrap();
    assert_eq!(
        result,
        Expression::Multiply {
            left: Box::new(Expression::Add {
                left: Box::new(Expression::Operand(Operand::Number(22))),
                right: Operand::Number(4)
            }),
            right: Operand::Number(9)
        }
    );

    let result = parser.parse("22 + (4 * 7) * 9").unwrap();
    assert_eq!(
        result,
        Expression::Multiply {
            left: Box::new(Expression::Add {
                left: Box::new(Expression::Operand(Operand::Number(22))),
                right: Operand::Expression(Box::new(Expression::Multiply {
                    left: Box::new(Expression::Operand(Operand::Number(4))),
                    right: Operand::Number(7)
                }))
            }),
            right: Operand::Number(9)
        }
    );
}

#[test]
fn test_evaluate() {
    let twenty_six = Expression::Add {
        left: Box::new(Expression::Operand(Operand::Number(22))),
        right: Operand::Number(4),
    };
    assert_eq!(evaluate(&twenty_six), 26);

    let four_hundred_and_fifty = Expression::Multiply {
        left: Box::new(Expression::Add {
            left: Box::new(Expression::Operand(Operand::Number(22))),
            right: Operand::Expression(Box::new(Expression::Multiply {
                left: Box::new(Expression::Operand(Operand::Number(4))),
                right: Operand::Number(7),
            })),
        }),
        right: Operand::Number(9),
    };
    assert_eq!(evaluate(&four_hundred_and_fifty), 450);
}

#[test]
fn test_grammar_part2() {
    let parser = grammar2::ExprParser::new();
    let _ = parser.parse("1 + 2").unwrap();
    let _ = parser.parse("1 + 2 + 3").unwrap();
    let parsed = parser.parse("1 + 2 * 3 + 4 * 5 + 6").unwrap();
    assert_eq!(evaluate2(&parsed), 231);
}
