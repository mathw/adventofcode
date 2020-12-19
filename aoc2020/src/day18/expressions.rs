#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expression {
    Add {
        left: Box<Expression>,
        right: Operand,
    },
    Multiply {
        left: Box<Expression>,
        right: Operand,
    },
    Operand(Operand),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Operand {
    Number(u32),
    Expression(Box<Expression>),
}

// part 2 types
pub enum Expr {
    Multiply(Box<Expr>, Factor),
    Factor(Factor),
}

pub enum Factor {
    Add(Box<Factor>, Term),
    Term(Term),
}

pub enum Term {
    Number(u64),
    Expr(Box<Expr>),
}
