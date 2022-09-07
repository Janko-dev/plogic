# Plogic
Propositional logic evaluator written in Rust. Plogic evaluates logic expressions in a REPL (Read, Execute, Print, Loop) enviroment and generates every truth table necessary to derive the final truth table result. This project is build purely out of recreational and educational purposes.

## Quick start
``` 
$ cargo run --release
Welcome to the REPL of Plogic.
Operator Usage:
    - And is '&'
    - Or is '|'
    - Not is '~'
    - Implication is '->'
    - Bi-implication is '<->'
Type a logic expression in the prompt :-)
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
The following grammar describes the parsing strategy to build the abstract syntax tree. It is noteworthy to mention that the usual propositional operator symbols are not used. The table below shows what each operator means.
| Operator | Meaning |
| -------- | ------- |
| `&` | The and-operator which says that both both left-hand side and right-hand side should evaluate to true, for the result to equal true as well. |
| `\|` | The or-operator which says that either or both left-hand side and right-hand side should evaluate to true, for the result to equal true as well.|
| `~` | The negation-operator flips true to false and false to true. |
| `->` | The conditional implication-operator only evaluates to false when the left-hand side is true and the right-hand side is false, otherwise the result is true. |
| `<->` | The biconditional implication-operator only evaluates to true when both left and right hand sides are equal to eachother.|

``` ebnf
Expression    = Conditional (("<->") Conditional)* ;
Conditional   = Or (("->") Or)* ;
Or            = And (("|") And)* ;
And           = Negation (("&") Negation)* ;
Negation      = "~" Negation | Primary ;
Primary       = Atom | "(" Expression ")" ;
Atom          = ["a"-"z" | "A"-"Z"]* ;
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
> p | (q & r) 
-------------------------------------------------------
[ r ] [ q ] [ p ] [ (q & r) ] [ q & r ] [ p | (q & r) ]
|---| |---| |---| |---------| |-------| |-------------|
| 0 | | 0 | | 0 | |    0    | |   0   | |      0      |
| 1 | | 0 | | 0 | |    0    | |   0   | |      0      |
| 0 | | 1 | | 0 | |    0    | |   0   | |      0      |
| 1 | | 1 | | 0 | |    1    | |   1   | |      1      |
| 0 | | 0 | | 1 | |    0    | |   0   | |      1      |
| 1 | | 0 | | 1 | |    0    | |   0   | |      1      |
| 0 | | 1 | | 1 | |    0    | |   0   | |      1      |
| 1 | | 1 | | 1 | |    1    | |   1   | |      1      |
-------------------------------------------------------
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