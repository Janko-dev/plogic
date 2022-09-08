# Plogic
Propositional logic evaluator and pattern transformer written in Rust. Plogic evaluates logic expressions in a REPL (Read, Execute, Print, Loop) environment and generates every truth table necessary to derive the final truth table result. The other use-case of Plogic is that you can define and apply certain pattern rules to transform the expression. This project is build purely out of recreational and educational purposes.

## Getting started
To build the binary, `cargo` is required alongside the `rust` installation. 
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
   -------------------
   | And     |  '&'  |
   | Or      |  '|'  |
   | Not     |  '~'  |
   | Cond    |  '->' |
   | Bi-Cond | '<->' |
   -------------------
   ----------------------
   | Rule       |  '=>' |
   | Derivation |       |
   ----------------------
   - help: usage info
   - ans:  previous answer
   - quit: exit repl
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
The following grammar describes the parsing strategy to build the abstract syntax tree. It is noteworthy to mention that the usual propositional operator symbols are not used (aside from conditional and bi-conditional). Instead, the operators come from the bitwise operators found in various programming languages. The table below shows what each operator means.
| Operator | Meaning |
| -------- | ------- |
| `&` | The and-operator which says that both both left-hand side and right-hand side should evaluate to true, for the result to equal true as well. |
| `\|` | The or-operator which says that either or both left-hand side and right-hand side should evaluate to true, for the result to equal true as well.|
| `~` | The negation-operator flips true to false and false to true. |
| `->` | The conditional implication-operator only evaluates to false when the left-hand side is true and the right-hand side is false, otherwise the result is true. |
| `<->` | The biconditional implication-operator only evaluates to true when both left and right hand sides are equal to eachother.|

Furthermore, to pattern match expressions and transform them, the `=>` is used after a valid proposition. Thereafter must follow a valid left hand-side expression, then a `=`, and then a valid right hand-side expression. Example:

`A & (B | C) => p & (q | r) = (p & q) | (p & r)`

This expression will produce the following. The expression `A & (B | C)` will be matched against the left hand-side after `=>`, i.e. `p & (q | r)`. Since both have the same pattern, it is a valid match. Thereafter, the right hand-side will be substituted according to the matched symbols in the left hand-side. Therefore, producing the following result:

`(A & B) | (A & C)`

``` ebnf
Expression       = Bi_conditional ("=>" Bi_conditional "=" Bi_conditional)? ;
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
> (p -> q) & (q -> p) 
-----------------------------------------------------------------------------------
[ q ] [ p ] [ (q -> p) ] [ (p -> q) ] [ q -> p ] [ p -> q ] [ (p -> q) & (q -> p) ]
|---| |---| |----------| |----------| |--------| |--------| |---------------------|
| 0 | | 0 | |    1     | |    1     | |   1    | |   1    | |          1          |
| 1 | | 0 | |    0     | |    1     | |   0    | |   1    | |          0          |
| 0 | | 1 | |    1     | |    0     | |   1    | |   0    | |          0          |
| 1 | | 1 | |    1     | |    1     | |   1    | |   1    | |          1          |
-----------------------------------------------------------------------------------
```