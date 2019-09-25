postfix
=======

`postfix` is a postfix math notation calculator. Postfix notation are
math expressions where the operators follow the operands. There is thus
no need for braces as the order of the operators enforce the order of
operations.

## Example

```rs
extern crate postfix;
use postfix::PostfixCalculator;

let calculator = PostfixCalculator::new();
println!("result: {:?}", calculator.evaluate(&vec!["4", "2", "+"]));
// result: 6.0
```
