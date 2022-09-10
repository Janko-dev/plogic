use std::{iter::Peekable, str::Chars};

    
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Identifier(String),     // [a-z/A-Z] 
    Not,                    // ~
    And,                    // &
    Or,                     // |
    Arrow,                  // ->
    TwinArrow,              // <->
    Rule,                   // =>
    Binding,                // :=
    OpenParen,              // (
    CloseParen,             // )
    Equal,                  // =
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

fn identifier(list: &mut Vec<Token>, input: &mut Peekable<Chars>, prev_input: &mut String){
    let mut lexeme = String::new();
    while let Some(c @ 'a'..='z') | Some(c @ 'A'..='Z') = input.peek() {
        lexeme.push(*c);
        input.next();
    }
    
    match lexeme.as_str() {
        "ans" => {
            if prev_input.len() > 0 {
                tokenize(list, prev_input, &mut "".to_string());
                return;
            }
        },
        "and" => list.push(Token::And),
        "or" =>  list.push(Token::Or),
        "not" => list.push(Token::Not),
        "implies" => list.push(Token::Arrow),
        "equiv" => list.push(Token::TwinArrow),
        "rule" => list.push(Token::Rule),
        _ => list.push(Token::Identifier(lexeme)),
    }
}

pub fn tokenize(list: &mut Vec<Token>, input: &mut String, prev_input: &mut String) {
    let mut input = input.chars().peekable();
    loop {
        match input.peek() {
            Some('&') => add_token(list, &mut input, Token::And),
            Some('|') => add_token(list, &mut input, Token::Or),
            Some('~') => add_token(list, &mut input, Token::Not),
            Some('(') => add_token(list, &mut input, Token::OpenParen),
            Some(')') => add_token(list, &mut input, Token::CloseParen),
            Some('=') => {
                input.next();
                if let Some('>') = input.peek() {
                    list.push(Token::Rule);
                    input.next();
                } else {
                    list.push(Token::Equal);
                }
            },
            Some(':') => {
                input.next();
                match input.next() {
                    Some('=') => list.push(Token::Binding),
                    other => println!("Unexpected character: expected '=', but found {:?}", other)
                }
            },
            Some('<') => add_twin_arrow(list, &mut input),
            Some('-') => add_arrow(list, &mut input),
            Some('a'..='z') | Some('A'..='Z') => identifier(list, &mut input, prev_input),

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
