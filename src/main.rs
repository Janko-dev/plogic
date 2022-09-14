use std::{io::{Write, self}, collections::HashMap};

use crate::{parser::{Expr, Rule}, lexer::Token};

mod lexer;
mod parser;
mod utils;
mod runtime;

fn main() {
    let mut input = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut prev_input: String = String::new();
    let mut rule_bindings: HashMap<String, Rule> = HashMap::new();
    let mut is_num_symbol: bool = true;

    println!("Welcome to the REPL of Plogic.");
    utils::usage();
    loop {
        input.clear();
        tokens.clear();

        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input).expect("Failed to read line from stdin");
        
        let mut input = input.trim().to_string();
        match input.as_str() {
            "help" => {
                utils::usage();
                continue;
            },
            "toggle" => {
                is_num_symbol = !is_num_symbol;
                if is_num_symbol {
                    println!("Changed truthtable symbols from 'T'/'F' to '1'/'0'");
                    continue;
                } else {
                    println!("Changed truthtable symbols from '1'/'0' to 'T'/'F'");
                    continue;
                }
            },
            "\n" | "" => continue,
            "quit" => break,
            _ => {},
        }

        lexer::tokenize(&mut tokens, &mut input, &mut prev_input);
        
        let mut interned: Vec<String> = Vec::new();
        let expr = parser::parse(&mut tokens, &mut interned);

        match expr {
            Ok(parser::Expr::Pattern(e, rule)) => {
                prev_input = match runtime::match_patterns(&*e, *rule, &interned, &rule_bindings) {
                    Ok(s) => {
                        println!("{}", s);
                        s
                    },
                    Err(msg) => {
                        println!("{}", msg);
                        "".to_string()
                    }
                }
            },
            Ok(parser::Expr::Binding(id, rule)) => {
                if let Expr::Primary(n) = *id {
                    rule_bindings.insert(interned[n].clone(), *rule);
                } else {
                    println!("Unreachable");
                }
            }
            Ok(e) => {
                let mut table = runtime::Table::new(&interned, &is_num_symbol);
                prev_input = utils::expr_to_string(&e, &interned);
                table.generate_truthtable(e);
                table.print();
            },
            Err(msg) => println!("{}", msg),
        }
    }

}
