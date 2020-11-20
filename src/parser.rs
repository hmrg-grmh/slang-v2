use std::fmt;

use crate::scanner::token::*;

use crate::eval::atom::Atom;

// https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

#[derive(Debug)]
pub enum S {
    Atom(Atom),
    Cons(Op, Vec<S>),
}

#[derive(Debug)]
pub enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Plus => "+",
                Op::Minus => "-",
                Op::Multiply => "*",
                Op::Divide => "/",
            }
        )
    }
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(i) => write!(f, "{}", i),
            S::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(mut tokens: Vec<Token>) -> Lexer {
        tokens.reverse();
        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens
            .pop()
            .unwrap_or_else(|| Token::from_ty(TokenType::EOF))
    }

    fn peek(&mut self) -> Token {
        self.tokens
            .last()
            .cloned()
            .unwrap_or_else(|| Token::from_ty(TokenType::EOF))
    }
}

pub fn expr(lexer: &mut Lexer) -> S {
    expr_bp(lexer, 0)
}

fn expr_bp(lexer: &mut Lexer, bp: u8) -> S {
    let nx = lexer.next();
    let mut lhs = match nx.ty {
        TokenType::Literal(a) => S::Atom(a),
        _ => panic!("Invalid token {}", nx),
    };

    loop {
        let nx = lexer.peek();
        let op = match nx.ty {
            TokenType::EOF | TokenType::WhiteSpace | TokenType::NewLine => break,
            TokenType::Plus => Op::Plus,
            TokenType::Minus => Op::Minus,
            TokenType::Slash => Op::Divide,
            TokenType::Star => Op::Multiply,
            _ => unimplemented!(), // could be panic
        };

        let (l_bp, r_bp) = infix_binding_power(&op);
        if l_bp < bp {
            break;
        }

        lexer.next();
        let rhs = expr_bp(lexer, r_bp);

        lhs = S::Cons(op, vec![lhs, rhs]);
    }

    lhs
}

fn infix_binding_power(op: &Op) -> (u8, u8) {
    match op {
        Op::Plus | Op::Minus => (1, 2),
        Op::Multiply | Op::Divide => (3, 4),
    }
}
