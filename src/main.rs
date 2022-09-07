use std::{iter::Peekable, str::Chars, slice::Iter, collections::HashMap, fmt::Display, io::{stdout, Write, self}};


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
    Pattern(Box<Expr>, Box<Expr>),          // Binary => Equivalence // r & s => p & q = q & p
    Equivalence(Box<Expr>, Box<Expr>),      // Binary = Binary// p & q = q & p
    Binary(Box<Expr>, BinOperator, Box<Expr>),
    Not(Box<Expr>),
    Group(Box<Expr>),
    Primary(usize),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Pattern(e, eq) => write!(f, "{} => {}", e, eq),
            Expr::Equivalence(l, r) => write!(f, "{} = {}", l, r),
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
    interned: Vec<String>,
    rows: usize,
}

impl Table {
    fn new() -> Self {
        Self { 
            map: HashMap::new(),
            interned: Vec::new(),
            rows: 0,
        }
    }

    fn fill_symbols(&mut self, expr: &Expr){
        match expr {
            Expr::Pattern(_e, _eq) => {
                todo!();
            },
            Expr::Equivalence(_left, _right) => {
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
            Expr::Pattern(_e, _eq) => {
                todo!();
            },
            Expr::Equivalence(_left, _right) => {
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
        
        for i in 0..self.rows{
            for (j, entry) in self.map.values_mut().enumerate() {
                let k = (2 as usize).pow((j+1) as u32);
                if i % k + 1 > k / 2 {
                    entry.push(1);
                } else {
                    entry.push(0);
                }
            }
        }

        self.eval(&expr);
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
            Expr::Pattern(e, eq) => format!("{} => {}", self.expr_to_string(e), self.expr_to_string(eq)),
            Expr::Equivalence(l, r) => format!("{} = {}", self.expr_to_string(l), self.expr_to_string(r)),
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
    let mut left = logic_twin_arrow(tokens, interned);
    if let Some(Token::Derive) = tokens.peek() {
        tokens.next();
        let right = expression(tokens, interned)?;
        left = Ok(Expr::Pattern(Box::new(left?), Box::new(right)))
    }
    left
}

fn expression(tokens: &mut Peekable<Iter<Token>>, interned: &mut Vec<String>) -> Result<Expr, String> {
    let mut left = logic_twin_arrow(tokens, interned);
    
    if let Some(Token::Equal) = tokens.peek() {
        tokens.next();
        let right = logic_twin_arrow(tokens, interned)?;
        left = Ok(Expr::Equivalence(Box::new(left?), Box::new(right)));
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

// creating rules that can be applied
// example: 
//          t & f ==> p & q = q & p
//          result: f & t

fn main() {
    let mut input = String::new();
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        input.clear();
        tokens.clear();

        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input).expect("Failed to read line from stdin");

        lexer(&mut tokens, &mut input.trim().to_string());
        // println!("{:?}", tokens);
        
        let mut table = Table::new();
        let expr = parse(&mut tokens, &mut table.interned);
        // println!("{:?}", expr);

        match expr {
            Ok(e) => {
                table.generate_truthtable(e);
                table.print();
            },
            Err(msg) => println!("{}", msg),
        }
    }

}
