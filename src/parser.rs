use std::{fmt::Display, iter::Peekable, slice::Iter};
use crate::lexer::Token;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum BinOperator {
    And,                    // &
    Or,                     // |
    Arrow,                  // ->
    TwinArrow,              // <->
}

impl Display for BinOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOperator::And => write!(f, "&"),
            BinOperator::Or => write!(f, "|"),
            BinOperator::Arrow => write!(f, "->"),
            BinOperator::TwinArrow => write!(f, "<->"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Rule {
    Equivalence(Expr, Expr),
    RuleId(usize)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Expr {
    Pattern(Box<Expr>, Box<Rule>),              // Binary => Equivalence // r & s => p & q = q & p
    Binding(Box<Expr>, Box<Rule>),              // switch := p & q = q & p // x & y => switch  
    Binary(Box<Expr>, BinOperator, Box<Expr>),
    Not(Box<Expr>),
    Group(Box<Expr>),
    Primary(usize),
}

pub fn parse(list: &mut Vec<Token>, interned: &mut Vec<String>) -> Result<Expr, String> {
    let mut tokens = list.iter().peekable();
    let res = pattern_match(&mut tokens, interned)?;
    if let Some(t) = tokens.peek() {
        Err(format!("Unexpected token, expected end of input: {:?}", t))
    } else {
        Ok(res)
    }
}

fn pattern_match(tokens: &mut Peekable<Iter<Token>>, interned: &mut Vec<String>) -> Result<Expr, String> {
    
    let mut left = logic_twin_arrow(tokens, interned);
    
    if let Some(Token::Binding) = tokens.peek() {
        
        if let Ok(Expr::Primary(_)) = left {
            tokens.next();
            let eq_lhs = logic_twin_arrow(tokens, interned)?;
            if let Some(Token::Equal) = tokens.peek() {
                tokens.next();
                let eq_rhs = logic_twin_arrow(tokens, interned)?;
                return Ok(Expr::Binding(Box::new(left?), Box::new(Rule::Equivalence(eq_lhs, eq_rhs))));
            } else {
                return Err("Expected '=' in pattern expression".to_string());
            }
        } else {
            return Err(format!("Can only bind rule to identifier, found {:?}", left));
        }
    }
    
    while let Some(Token::Rule) = tokens.peek() {
        tokens.next();
        let eq_lhs = logic_twin_arrow(tokens, interned)?;
        match tokens.peek() {
            Some(Token::Equal) => {
                tokens.next();
                let eq_rhs = logic_twin_arrow(tokens, interned)?;
                // (p & q => x & y = y & x)
                left = Ok(Expr::Pattern(Box::new(left?), Box::new(Rule::Equivalence(eq_lhs, eq_rhs))));
            },
            Some(other) => return Err(format!("Expected '=' in pattern expression or rule identifier, found {:?}", other)),
            None => {
                if let Expr::Primary(n) = eq_lhs {
                    return Ok(Expr::Pattern(Box::new(left?), Box::new(Rule::RuleId(n))));
                } else {
                    return Err(format!("Expected rule identifier name, but got {:?}", eq_lhs));
                }
            }
        }
    }
    left
}

fn logic_twin_arrow(tokens: &mut Peekable<Iter<Token>>, interned: &mut Vec<String>) -> Result<Expr, String> {
    let mut left = logic_arrow(tokens, interned);

    while let Some(Token::TwinArrow) = tokens.peek() {
        tokens.next();
        let right = logic_arrow(tokens, interned)?;
        left = Ok(Expr::Binary(Box::new(left?), BinOperator::TwinArrow, Box::new(right)));
    }
    left
}

fn logic_arrow(tokens: &mut Peekable<Iter<Token>>, interned: &mut Vec<String>) -> Result<Expr, String> {
    let mut left = logic_or(tokens, interned);

    while let Some(Token::Arrow) = tokens.peek() {
        tokens.next();
        let right = logic_or(tokens, interned)?;
        left = Ok(Expr::Binary(Box::new(left?), BinOperator::Arrow, Box::new(right)));
    }
    left
}

fn logic_or(tokens: &mut Peekable<Iter<Token>>, interned: &mut Vec<String>) -> Result<Expr, String> {
    let mut left = logic_and(tokens, interned);

    while let Some(Token::Or) = tokens.peek() {
        tokens.next();
        let right = logic_and(tokens, interned)?;
        left = Ok(Expr::Binary(Box::new(left?), BinOperator::Or, Box::new(right)));
    }
    left
}

fn logic_and(tokens: &mut Peekable<Iter<Token>>, interned: &mut Vec<String>) -> Result<Expr, String> {
    let mut left = logic_not(tokens, interned);

    while let Some(Token::And) = tokens.peek() {
        tokens.next();
        let right = logic_not(tokens, interned)?;
        left = Ok(Expr::Binary(Box::new(left?), BinOperator::And, Box::new(right)));
    }
    left
}

fn logic_not(tokens: &mut Peekable<Iter<Token>>, interned: &mut Vec<String>) -> Result<Expr, String> {
    
    if let Some(Token::Not) = tokens.peek() {
        tokens.next();
        return Ok(Expr::Not(Box::new(logic_not(tokens, interned)?)));
    }
    primary(tokens, interned)
}

fn primary(tokens: &mut Peekable<Iter<Token>>, interned: &mut Vec<String>) -> Result<Expr, String> {
    match tokens.next() {
        Some(Token::Identifier(s)) => {
            let index = interned.iter().position(|i| i == s);
            match index {
                Some(n) => Ok(Expr::Primary(n)),
                None => {
                    interned.push(s.to_string());
                    Ok(Expr::Primary(interned.len()-1))
                }
            }
        },
        Some(Token::OpenParen) => {
            let expr = logic_twin_arrow(tokens, interned)?;
            if let Some(Token::CloseParen) = tokens.next() {
                Ok(Expr::Group(Box::new(expr)))
            } else {
                Err("Missing closing parenthesis".to_string())
            }
        },
        Some(other) => {
            Err(format!("Unexpected token: {:?}", other))
        },
        None => Err("Unexpected token: None".to_string()),
    }
}