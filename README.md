# Plogic
Propositional logic evaluator and pattern transformer written in Rust. Plogic evaluates logic expressions in a REPL (Read, Execute, Print, Loop) environment and generates every truth table necessary to derive the final truth table result. Another use-case of Plogic is to define and apply pattern rules to transform a expression into another expression. This is a common operation in proof systems. Plogic is a very basic version of a proof system.

This project is build purely out of recreational and educational purposes.

## Getting started
To build the binary executable, `cargo` is required alongside the [Rust](https://www.rust-lang.org/tools/install) installation.
```
$ git clone https://github.com/Janko-dev/plogic.git
$ cd plogic
$ cargo build --release
```
Go to `target/release/` to find the `plogic.exe` (on windows). Run the executable to enter the REPL environment.

``` 
$ cargo run --release
Welcome to the REPL of Plogic.
Usage:
   ----------------------------------------
   | And oprator      |  '&'  | 'and'     |
   | Or oprator       |  '|'  | 'or'      |
   | Not oprator      |  '~'  | 'not'     |
   | Cond oprator     |  '->' | 'implies' |
   | Bi-Cond oprator  | '<->' | 'equiv'   |
   ----------------------------------------
   -----------------------------------------------------------------------
   | Rule       |  ':='  | identifier-name := lhs-pattern = rhs-pattern  |
   | binding    | 'rule' | example:   commutative := p & q = q & p       |
   -----------------------------------------------------------------------
   | Rule       |  '=>'  | inline pattern: A & B => p & q = q & p        |
   | Derivation |        | bound pattern : A & B => commutative          |
   -----------------------------------------------------------------------
   - help:   usage info
   - ans:    previous answer
   - toggle: toggle between (T/F) and (1/0) in truth tables
   - quit:   exit repl
> p & q
---------------------
[ q ] [ p ] [ p & q ]
|---| |---| |-------|
| 0 | | 0 | |   0   |
| 1 | | 0 | |   0   |
| 0 | | 1 | |   0   |
| 1 | | 1 | |   1   |
---------------------
>
```

## Grammar
The following grammar describes the parsing strategy to build the abstract syntax tree. It is noteworthy to mention that the usual mathematical symbols for the operators are not used. Instead, the operators come from the bitwise operators found in various programming languages and optional keywords which may be used for the sake of convenience. The table below shows what each operator means.
| Operator | Meaning |
| -------- | ------- |
| `&` or `and` | The and-operator which says that both left-hand side and right-hand side should evaluate to true, for the result to equal true as well. |
| `\|` or `or` | The or-operator which says that either or both left-hand side and right-hand side should evaluate to true, for the result to equal true as well.|
| `~` or `not` | The negation-operator flips true to false and false to true. |
| `->` or `implies` | The conditional implication-operator only evaluates to false when the left-hand side is true and the right-hand side is false, otherwise the result is true. |
| `<->` or `equiv` | The biconditional implication-operator only evaluates to true when both left and right hand sides are equal to eachother.|

## Rule-based pattern matching
Furthermore, to pattern match expressions and transform them into other expressions, the `=>` or `rule` keyword is used after a valid propositional expression. Thereafter must follow a valid left hand-side expression, then a `=`, and then a valid right hand-side expression. Example:

`A & (B | C) => p & (q | r) = (p & q) | (p & r)`

This expression describes the following. The expression `A & (B | C)` will be matched against the left hand-side after `=>`, i.e. `p & (q | r)`. Since both have the same pattern, it is a valid match. Thereafter, the right hand-side will be substituted according to the matched symbols in the left hand-side. Therefore, producing the following result:

`(A & B) | (A & C)`

Aside from using inline rule patterns as demonstrated above, which can get convoluted to type if the given rule pattern is used multiple times, there is also the possibility to bind a rule to an identifier name. Consider the following:

`distributive := p & (q | r) = (p & q) | (p & r)`

the identifier name `distributive` is now bound to the pattern given after `:=`. The pattern identifier can now be used instead. i.e., 

`A & (B | C) => distributive`

which produces the same result as before, i.e., `(A & B) | (A & C)`.

``` ebnf
Expression       = Rule_binding | Rule_apply ;
Rule_binding     = Atom ":=" Bi_conditional "=" Bi_conditional ;
Rule_apply       = Bi_conditional ("=>" Bi_conditional "=" Bi_conditional)? ;
Bi_conditional   = Conditional (("<->") Conditional)* ;
Conditional      = Or (("->") Or)* ;
Or               = And (("|") And)* ;
And              = Negation (("&") Negation)* ;
Negation         = "~" Negation | Primary ;
Primary          = Atom | "(" Bi_conditional ")" ;
Atom             = ["a"-"z" | "A"-"Z"]* ;
```

## More Examples
 
```
> A & ~A
-----------------------
[ A ] [ ~A ] [ A & ~A ]
|---| |----| |--------|
| 0 | | 1  | |   0    |
| 1 | | 0  | |   0    |
-----------------------
```
```
> (p implies q) and (q implies p) 
-----------------------------------------------------------------------------------
[ q ] [ p ] [ (q -> p) ] [ (p -> q) ] [ q -> p ] [ p -> q ] [ (p -> q) & (q -> p) ]
|---| |---| |----------| |----------| |--------| |--------| |---------------------|
| 0 | | 0 | |    1     | |    1     | |   1    | |   1    | |          1          |
| 1 | | 0 | |    0     | |    1     | |   0    | |   1    | |          0          |
| 0 | | 1 | |    1     | |    0     | |   1    | |   0    | |          0          |
| 1 | | 1 | |    1     | |    1     | |   1    | |   1    | |          1          |
-----------------------------------------------------------------------------------
```
```
> A & (B | C) => p & (q | r) = (p & q) | (p & r)
(A & B) | (A & C)
> ans
-----------------------------------------------------------
[ C ] [ B ] [ A ] [ A & C ] [ A & B ] [ (A & B) | (A & C) ]
|---| |---| |---| |-------| |-------| |-------------------|
| 0 | | 0 | | 0 | |   0   | |   0   | |         0         |
| 0 | | 0 | | 1 | |   0   | |   0   | |         0         |
| 0 | | 1 | | 0 | |   0   | |   0   | |         0         |
| 0 | | 1 | | 1 | |   0   | |   1   | |         1         |
| 1 | | 0 | | 0 | |   0   | |   0   | |         0         |
| 1 | | 0 | | 1 | |   1   | |   0   | |         1         |
| 1 | | 1 | | 0 | |   0   | |   0   | |         0         |
| 1 | | 1 | | 1 | |   1   | |   1   | |         1         |
-----------------------------------------------------------
>
```
```

```
