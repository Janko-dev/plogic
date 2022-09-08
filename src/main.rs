use std::{iter::Peekable, str::Chars, slice::Iter, collections::HashMap, fmt::Display, io::{Write, self}};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Token {
    Identifier(String),     // [a-z/A-Z] 
    Not,                    // ~
    And,                    // &
    Or,                     // |
    Arrow,                  // ->
    TwinArrow,              // <->
    Derive,                 // =>
    OpenParen,              // (
    CloseParen,             // )
    Equal,                  // =
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum BinOperator {
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
enum Expr {
    Pattern(Box<Expr>, Box<Expr>, Box<Expr>),          // Binary => Equivalence // r & s => p & q = q & p
    Binary(Box<Expr>, BinOperator, Box<Expr>),
    Not(Box<Expr>),
    Group(Box<Expr>),
    Primary(usize),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Pattern(e, lhs, rhs) => write!(f, "{} => {} = {}", e, lhs, rhs),
            Expr::Binary(l, op, r) => write!(f, "{} {} {}", l, op, r),
            Expr::Not(e) => write!(f, "~{}", e),
            Expr::Group(e) => write!(f, "({})", e),
            Expr::Primary(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug)]
struct Table {
    map: HashMap<Expr, Vec<usize>>,
    patterns: HashMap<Expr, Expr>,
    interned: Vec<String>,
    rows: usize,
}

impl Table {
    fn new() -> Self {
        Self { 
            map: HashMap::new(),
            patterns: HashMap::new(),
            interned: Vec::new(),
            rows: 0,
        }
    }

    fn fill_symbols(&mut self, expr: &Expr){
        match expr {
            Expr::Pattern(_e, _l, _r) => {
                todo!();
            },
            Expr::Binary(left, _, right) => {
                self.fill_symbols(&*left);
                self.fill_symbols(&*right);
            },
            Expr::Not(e) => {
                self.fill_symbols(&*e);
            },
            Expr::Group(e) => {
                self.fill_symbols(&*e);
            },
            e @ Expr::Primary(_) => {
                if let None = self.map.get(&e) {
                    self.map.insert(e.clone(), Vec::new());
                }
            },
        }
    }

    fn zipped_operation<F: Fn((&usize, &usize)) -> usize>(left: Vec<usize>, right: Vec<usize>, f: F) -> Vec<usize> {
        left
        .iter()
        .zip(right.iter())
        .map(f)
        .collect::<Vec<usize>>()
    }

    fn eval(&mut self, expr: &Expr) -> Vec<usize> {
        match expr {
            Expr::Pattern(_e, _l, _r) => {
                todo!();
            },
            e @ Expr::Binary(l, op, r) => {
                let left = self.eval(&*l);
                let right = self.eval(&*r);
                
                let res = match op {
                    BinOperator::And => Table::zipped_operation(left, right, |(a, b)| a & b),
                    BinOperator::Or => Table::zipped_operation(left, right, |(a, b)| a | b),
                    BinOperator::Arrow => Table::zipped_operation(left, right, |(a, b)| if a.eq(&1) && b.eq(&0) { 0 } else { 1 }),
                    BinOperator::TwinArrow => Table::zipped_operation(left, right, |(a, b)| a.eq(b) as usize),
                };
                
                if let None = self.map.get(&e) {
                    self.map.insert(e.clone(), res.clone());
                }
                res
            },
            e @ Expr::Not(inner) => {
                let res = self.eval(&*inner)
                                          .iter()
                                          .map(|x| if x.eq(&0) { 1 } else { 0 })
                                          .collect::<Vec<usize>>();
                if let None = self.map.get(&e) {
                    self.map.insert(e.clone(), res.clone());
                }
                res
            },
            e @ Expr::Group(inner) => {
                let res = self.eval(&*inner);
                if let None = self.map.get(&e) {
                    self.map.insert(e.clone(), res.clone());
                }
                res
            },
            e @ Expr::Primary(_) => {
                self.map.get(e).unwrap().to_vec()
            },
        }
    }

    fn generate_truthtable(&mut self, expr: Expr){
        self.fill_symbols(&expr);

        let count = self.map.len();
        self.rows = (2 as usize).pow(count as u32);   
        
        // for (j, entry) in self.map.iter().collect::<Vec<_>>() {
        //     entry.push(1);
        // }

        for (j, entry) in self.map.values_mut().enumerate() {
            for i in 0..self.rows{
                let k = (2 as usize).pow((j+1) as u32);
                if i % k + 1 > k / 2 {
                    entry.push(1);
                } else {
                    entry.push(0);
                }
            }
        }

        self.eval(&expr);
        self.map.retain(|k, _| {
            if let Expr::Group(_) = k {
                false
            } else {
                true
            }
        });

    }

    fn sort(&self) -> Vec<(&Expr, &Vec<usize>)> {
        let mut res = self.map
            .iter()
            .collect::<Vec<_>>();
        res.sort_by(|x, y| y.0.cmp(&x.0));
        res
    }

    fn expr_to_string(&self, expr: &Expr) -> String {
        match expr {
            Expr::Pattern(e, l, r) => 
                format!("{} => {} = {}", self.expr_to_string(e), self.expr_to_string(l), self.expr_to_string(r)),
            Expr::Binary(l, op, r) => format!("{} {} {}", self.expr_to_string(l), op, self.expr_to_string(r)),
            Expr::Not(e) => format!("~{}", self.expr_to_string(e)),
            Expr::Group(e) => format!("({})", self.expr_to_string(e)),
            Expr::Primary(n) => format!("{}", self.interned[*n]),
        }
    }

    fn print(&self) {
        let mut total = 0;
        let mut headers: Vec<String> = Vec::new();
        let list = self.sort();
        for (expr, _) in  &list {
            let head = format!("[ {} ] ", self.expr_to_string(expr));
            total += head.len();
            headers.push(head);
        }

        println!("{:-<1$}", "", total-1);
        for head in &headers {
            print!("{}", head);
        }
        println!();
        
        for head in &headers {
            print!("|{:-<1$}| ", "", head.len()-3);
        }
        println!();

        for i in 0..self.rows {
            for (j, head) in headers.iter().enumerate() {
                let len = head.len();
                if len % 2 == 0 {
                    print!("|{: <1$}", "", len/2-2);
                    print!("{}", list[j].1[i]);
                    print!("{: <1$}| ", "", len/2-2);
                } else {
                    print!("|{: <1$}", "", len/2-2);
                    print!("{}", list[j].1[i]);
                    print!("{: <1$}| ", "", len/2-1);
                }
            }
            println!();
        }
        println!("{:-<1$}", "", total-1);
    }

    fn traverse_and_match(&mut self, expr: Expr, lhs: Expr) {
        match lhs {
            Expr::Binary(pat_left, pat_op, pat_right) => {
                if let Expr::Binary(e_left, e_op, e_right) = expr {
                    if pat_op == e_op {
                        self.traverse_and_match(*e_left, *pat_left);
                        self.traverse_and_match(*e_right, *pat_right);
                    } else {
                        println!("Expression does not match pattern");
                    }
                } else {
                    println!("Expression does not match pattern: {}", expr);
                }
            },
            Expr::Not(pat_e) => {
                if let Expr::Not(e) = expr {
                    self.traverse_and_match(*e, *pat_e);
                } else {
                    println!("Expression does not match pattern: {}", expr);
                }
            },
            Expr::Group(pat_e) => {
                if let Expr::Group(e) = expr {
                    self.traverse_and_match(*e, *pat_e);
                } else {
                    println!("Expression does not match pattern: {}", expr);
                }
            },
            Expr::Primary(_) => {
                if let None = self.patterns.get(&lhs) {
                    self.patterns.insert(lhs.clone(), expr);
                }
            },
            _ => {
                println!("Unreachable");
            }
        }
    }

    fn subsitute_in(&mut self, expr: Expr) -> Result<Expr, String> {
        match expr {
            Expr::Binary(l, op, r) => {
                let left = self.subsitute_in(*l)?;
                let right = self.subsitute_in(*r)?;
                Ok(Expr::Binary(Box::new(left), op, Box::new(right)))
            },
            Expr::Not(e) => {
                let res = self.subsitute_in(*e)?;
                Ok(Expr::Not(Box::new(res)))
            },
            Expr::Group(e) => {
                let res = self.subsitute_in(*e)?;
                Ok(Expr::Group(Box::new(res)))
            },
            Expr::Primary(_) => {
                if let Some(v) = self.patterns.get(&expr) {
                    Ok(v.clone())
                } else {
                    Err("Pattern could not be found in expression or left hand side".to_string())
                }
            },
            _ => Err("Unreachable".to_string())
        }
    }

    fn match_patterns(&mut self, expr: Expr, lhs: Expr, rhs: Expr) -> Result<String, String> {
        self.traverse_and_match(expr, lhs);
        let result = self.subsitute_in(rhs);
        match result {
            Ok(e) => Ok(self.expr_to_string(&e)),
            Err(s) => Ok(s),
        }
    }

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

    if lexeme.eq("ans") && prev_input.len() > 0{
        lexer(list, prev_input, &mut "".to_string());
        return;
    } 

    list.push(Token::Identifier(lexeme));
}

fn lexer(list: &mut Vec<Token>, input: &mut String, prev_input: &mut String) {
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
                    list.push(Token::Derive);
                    input.next();
                } else {
                    list.push(Token::Equal);
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

fn parse(list: &mut Vec<Token>, interned: &mut Vec<String>) -> Result<Expr, String> {
    let mut tokens = list.iter().peekable();
    
    let res = pattern_match(&mut tokens, interned)?;
    if let Some(t) = tokens.peek() {
        Err(format!("Unexpected token, expected end of input: {:?}", t))
    } else {
        Ok(res)
    }
}

fn pattern_match(tokens: &mut Peekable<Iter<Token>>, interned: &mut Vec<String>) -> Result<Expr, String> {
    let left = logic_twin_arrow(tokens, interned);
    if let Some(Token::Derive) = tokens.peek() {
        tokens.next();
        let eq_lhs = logic_twin_arrow(tokens, interned)?;
        if let Some(Token::Equal) = tokens.peek() {
            tokens.next();
            let eq_rhs = logic_twin_arrow(tokens, interned)?;
            return Ok(Expr::Pattern(Box::new(left?), Box::new(eq_lhs), Box::new(eq_rhs)));
        } else {
            return Err("Expected '=' in pattern expression".to_string());
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

fn usage(){
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

fn main() {
    let mut input = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut prev_input: String = String::new();
    println!("Welcome to the REPL of Plogic.");
    usage();
    loop {
        input.clear();
        tokens.clear();

        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input).expect("Failed to read line from stdin");
        
        let mut input = input.trim().to_string();
        match input.as_str() {
            "help" => {
                usage();
                continue;
            },
            "\n" | "" => continue,
            "quit" => break,
            _ => {},
        }

        lexer(&mut tokens, &mut input, &mut prev_input);
        // println!("{:?}", tokens);
        
        let mut table = Table::new();
        let expr = parse(&mut tokens, &mut table.interned);
        // println!("{:?}", expr);

        match expr {
            Ok(Expr::Pattern(e, lhs, rhs)) => {
                // (t | f) & f => p & q = q & p
                prev_input = match table.match_patterns(*e, *lhs, *rhs) {
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
            Ok(e) => {
                prev_input = table.expr_to_string(&e);
                table.generate_truthtable(e);
                table.print();
            },
            Err(msg) => println!("{}", msg),
        }
    }

}
