use crate::parser::{Expr, Rule};

pub fn usage(){
    println!("Usage:");
    println!("   -------------------");
    println!("   | And     |  '&'  |");
    println!("   | Or      |  '|'  |");
    println!("   | Not     |  '~'  |");
    println!("   | Cond    |  '->' |");
    println!("   | Bi-Cond | '<->' |");
    println!("   -------------------");
    println!("   ----------------------");
    println!("   | Rule       |  '=>' |");
    println!("   | Derivation |       |");
    println!("   ----------------------");
    println!("   - help: usage info");
    println!("   - ans:  previous answer");
    println!("   - quit: exit repl");
}

fn rule_to_string(rule: &Rule, interned: &Vec<String>) -> String {
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