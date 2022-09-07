use std::{iter::Peekable, str::Chars, slice::Iter};


#[derive(Debug)]
enum Token {
    Identifier(String),
    Not,
    And,
    Or,
    Arrow,
    TwinArrow,
    
    OpenParen,
    CloseParen,
    Equal,
}

#[derive(Debug)]
enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Group(Box<Expr>),
    Primary(String),
}

fn add_token(list: &mut Vec<Token>, input: &mut Peekable<Chars>, token: Token) {
    list.push(token);
    input.next();
}

fn add_arrow(list: &mut Vec<Token>, input: &mut Peekable<Chars>){
    input.next();
    match input.peek() {
        Some('>') => {
            input.next();
            list.push(Token::Arrow);
        },
        Some(other) => {
            println!("Unexpected character: {}", other);
            input.next();
        },
        None => {
            println!("Unexpected character: None");
            input.next();
        }
    }
}

fn add_twin_arrow(list: &mut Vec<Token>, input: &mut Peekable<Chars>){
    
    for c in "<->".chars(){
        if c != *input.peek().unwrap() {
            println!("Unexpected character: {}", c);
            input.next();
            return;
        }
        input.next();
    }
    list.push(Token::TwinArrow);
}

fn identifier(list: &mut Vec<Token>, input: &mut Peekable<Chars>){
    let mut lexeme = String::new();
    while let Some(c @ 'a'..='z') | Some(c @ 'A'..='Z') = input.peek() {
        lexeme.push(*c);
        input.next();
    }

    list.push(Token::Identifier(lexeme));
}

fn lexer(list: &mut Vec<Token>, input: &mut String) {
    let mut input = input.chars().peekable();
    loop {
        match input.peek() {
            Some('&') => add_token(list, &mut input, Token::And),
            Some('|') => add_token(list, &mut input, Token::Or),
            Some('~') => add_token(list, &mut input, Token::Not),
            Some('(') => add_token(list, &mut input, Token::OpenParen),
            Some(')') => add_token(list, &mut input, Token::CloseParen),
            Some('=') => add_token(list, &mut input, Token::Equal),

            Some('<') => add_twin_arrow(list, &mut input),
            Some('-') => add_arrow(list, &mut input),
            Some('a'..='z') | Some('A'..='Z') => identifier(list, &mut input),

            Some(' ') | Some('\t') | Some('\n') => {
                input.next();
            },
            
            Some(other) => {
                println!("Unexpected character: {}", other);
                input.next();
            },
            None => break,
        }
    }

}

fn parse(list: &mut Vec<Token>) -> Result<Expr, String> {

    let mut tokens = list.iter().peekable();
    expression(&mut tokens)
}

fn expression(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    let mut left = logic_not(tokens);

    while let Some(Token::Equal) = tokens.peek() {
        tokens.next();
        let right = logic_not(tokens)?;
        left = Ok(Expr::Binary(Box::new(left?), Token::Equal, Box::new(right)));
    }
    left
}

fn logic_not(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    
    while let Some(Token::Not) = tokens.peek() {
        tokens.next();
        return Ok(Expr::Unary(Token::Not, Box::new(logic_not(tokens)?)));
    }
    logic_and(tokens)
}

fn logic_and(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    let mut left = logic_or(tokens);

    while let Some(Token::And) = tokens.peek() {
        tokens.next();
        let right = logic_or(tokens)?;
        left = Ok(Expr::Binary(Box::new(left?), Token::And, Box::new(right)));
    }
    left
}

fn logic_or(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    let mut left = logic_arrow(tokens);

    while let Some(Token::Or) = tokens.peek() {
        tokens.next();
        let right = logic_arrow(tokens)?;
        left = Ok(Expr::Binary(Box::new(left?), Token::Or, Box::new(right)));
    }
    left
}

fn logic_arrow(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    let mut left = logic_twin_arrow(tokens);

    while let Some(Token::Arrow) = tokens.peek() {
        tokens.next();
        let right = logic_twin_arrow(tokens)?;
        left = Ok(Expr::Binary(Box::new(left?), Token::Arrow, Box::new(right)));
    }
    left
}

fn logic_twin_arrow(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    let mut left = primary(tokens);

    while let Some(Token::TwinArrow) = tokens.peek() {
        tokens.next();
        let right = primary(tokens)?;
        left = Ok(Expr::Binary(Box::new(left?), Token::TwinArrow, Box::new(right)));
    }
    left
}

fn primary(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    match tokens.next() {
        Some(Token::Identifier(s)) => Ok(Expr::Primary(s.clone())),
        Some(Token::OpenParen) => {
            let expr = logic_not(tokens)?;
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
// ideas: truth table generator from parsed proposition
// example: p & q
// | p | q |
// |---|---|
// | 0 | 0 |
// | 0 | 1 |
// | 1 | 0 |
// | 1 | 1 |


// creating rules that can be applied
// example: 
//          t & f ==> p & q = q & p
//          result: f & t

fn main() {
    let mut input = "~p & q = q & p".to_string();
    // let mut input = "p & (q | r) = (p & q) | (p & r)".to_string();
    
    let mut tokens: Vec<Token> = Vec::new();
    lexer(&mut tokens, &mut input);
    println!("{:?}", tokens);
    let expr = parse(&mut tokens);
    println!("{:?}", expr);
}
