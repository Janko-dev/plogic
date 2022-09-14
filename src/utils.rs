use crate::parser::{Expr, Rule};

pub fn usage(){
    println!("Usage:");
    println!("   ----------------------------------------");
    println!("   | And oprator      |  '&'  | 'and'     |");
    println!("   | Or oprator       |  '|'  | 'or'      |");
    println!("   | Not oprator      |  '~'  | 'not'     |");
    println!("   | Cond oprator     |  '->' | 'implies' |");
    println!("   | Bi-Cond oprator  | '<->' | 'equiv'   |");
    println!("   ----------------------------------------");
    println!("   -----------------------------------------------------------------------");
    println!("   | Rule       |  ':='  | identifier-name := lhs-pattern = rhs-pattern  |");
    println!("   | binding    | 'rule' | example:   commutative := p & q = q & p       |");
    println!("   -----------------------------------------------------------------------");
    println!("   | Rule       |  '=>'  | inline pattern: A & B => p & q = q & p        |");
    println!("   | Derivation |        | bound pattern : A & B => commutative          |");
    println!("   -----------------------------------------------------------------------");
    println!("   - help:   usage info");
    println!("   - ans:    previous answer");
    println!("   - toggle: toggle between (T/F) and (1/0) in truth tables");
    println!("   - quit:   exit repl");
}

pub fn rule_to_string(rule: &Rule, interned: &Vec<String>) -> String {
    match rule {
        Rule::Equivalence(lhs, rhs) => format!("{} = {}", expr_to_string(lhs, interned), expr_to_string(rhs, interned)),
        Rule::RuleId(n) => format!("{}", interned[*n])
    }
}

pub fn expr_to_string(expr: &Expr, interned: &Vec<String>) -> String {
    match expr {
        Expr::Pattern(e, rule) => 
            format!("{} => {}", expr_to_string(e, interned), rule_to_string(rule, interned)),
        Expr::Binding(id, rule) => 
            format!("{} := {}", expr_to_string(id, interned), rule_to_string(rule, interned)),
        Expr::Binary(l, op, r) => format!("{} {} {}", expr_to_string(l, interned), op, expr_to_string(r, interned)),
        Expr::Not(e) => format!("~{}", expr_to_string(e, interned)),
        Expr::Group(e) => format!("({})", expr_to_string(e, interned)),
        Expr::Primary(n) => format!("{}", interned[*n]),
    }
}