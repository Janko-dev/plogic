use std::{collections::HashMap};
use crate::{parser::{Expr, Rule, BinOperator}, utils};

#[derive(Debug)]
pub struct Table {
    map: HashMap<Expr, Vec<usize>>,
    atoms: Vec<Expr>,
    pub interned: Vec<String>,
    rows: usize,
}

impl Table {
    pub fn new(interned: &Vec<String>) -> Self {
        Self { 
            map: HashMap::new(),
            atoms: Vec::new(),
            interned: interned.clone(),
            rows: 0,
        }
    }

    fn fill_symbols(&mut self, expr: &Expr){
        match expr {
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
                    self.atoms.push(e.clone());
                }
            },
            other => println!("Unreachable: found {}", utils::expr_to_string(other, &self.interned))
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
            _ => panic!("Unreachable eval"),
        }
    }

    pub fn generate_truthtable(&mut self, expr: Expr){
        self.fill_symbols(&expr);

        let count = self.map.len();
        self.rows = (2 as usize).pow(count as u32);   
        
        self.atoms.sort_by(|a, b| a.cmp(&b));
        for (j, e) in self.atoms.iter().enumerate() {
            let mut entry: Vec<usize> = Vec::new();
            for i in 0..self.rows{
                let k = (2 as usize).pow((j+1) as u32);
                if i % k + 1 > k / 2 {
                    entry.push(1);
                } else {
                    entry.push(0);
                }
            }
            self.map.insert(e.clone(), entry);
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

    pub fn print(&self) {
        let mut total = 0;
        let mut headers: Vec<String> = Vec::new();
        let list = self.sort();
        for (expr, _) in  &list {
            let head = format!("[ {} ] ", utils::expr_to_string(expr, &self.interned));
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

pub fn match_patterns(expr: Expr, rule: Rule, interned: &Vec<String>, rule_bindings: &HashMap<String, Rule>) -> Result<String, String> {
    let mut patterns: HashMap<Expr, Expr> = HashMap::new();
    let result = match rule {
        Rule::Equivalence(lhs, rhs) => {
            traverse_and_match(expr, lhs, &mut patterns);
            subsitute_in(rhs, &mut patterns)
        },
        Rule::RuleId(n) => {
            let rule_maybe = rule_bindings.get(&interned[n]);
            if let Some(Rule::Equivalence(lhs, rhs)) = rule_maybe {
                traverse_and_match(expr, lhs.clone(), &mut patterns);
                subsitute_in(rhs.clone(), &mut patterns)
            } else {
                return Err("Undefined rule used".to_string());
            }
        }
    };

    match result {
        Ok(e) => Ok(utils::expr_to_string(&e, interned)),
        Err(s) => Ok(s),
    }
}

fn traverse_and_match(expr: Expr, lhs: Expr, patterns: &mut HashMap<Expr, Expr>) {
    match lhs {
        Expr::Binary(pat_left, pat_op, pat_right) => {
            if let Expr::Binary(e_left, e_op, e_right) = expr {
                if pat_op == e_op {
                    traverse_and_match(*e_left, *pat_left, patterns);
                    traverse_and_match(*e_right, *pat_right, patterns);
                } else {
                    println!("Expression does not match pattern");
                }
            } else {
                println!("Expression does not match pattern: {:?}", expr);
            }
        },
        Expr::Not(pat_e) => {
            if let Expr::Not(e) = expr {
                traverse_and_match(*e, *pat_e, patterns);
            } else {
                println!("Expression does not match pattern: {:?}", expr);
            }
        },
        Expr::Group(pat_e) => {
            if let Expr::Group(e) = expr {
                traverse_and_match(*e, *pat_e, patterns);
            } else {
                println!("Expression does not match pattern: {:?}", expr);
            }
        },
        Expr::Primary(_) => {
            if let None = patterns.get(&lhs) {
                patterns.insert(lhs.clone(), expr);
            }
        },
        _ => {
            println!("Unreachable");
        }
    }
}

fn subsitute_in(expr: Expr, patterns: &mut HashMap<Expr, Expr>) -> Result<Expr, String> {
    match expr {
        Expr::Binary(l, op, r) => {
            let left = subsitute_in(*l, patterns)?;
            let right = subsitute_in(*r, patterns)?;
            Ok(Expr::Binary(Box::new(left), op, Box::new(right)))
        },
        Expr::Not(e) => {
            let res = subsitute_in(*e, patterns)?;
            Ok(Expr::Not(Box::new(res)))
        },
        Expr::Group(e) => {
            let res = subsitute_in(*e, patterns)?;
            Ok(Expr::Group(Box::new(res)))
        },
        Expr::Primary(_) => {
            if let Some(v) = patterns.get(&expr) {
                Ok(v.clone())
            } else {
                Err("Pattern could not be found in expression or left hand side".to_string())
            }
        },
        _ => Err("Unreachable".to_string())
    }
}