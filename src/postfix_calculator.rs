use super::operator::Operator;
use std::collections::HashMap;

pub struct PostfixCalculator<'a> {
    operators: HashMap<&'a str, Operator<'a>>,
}

impl<'a> PostfixCalculator<'a> {
    pub fn new() -> Self {
        let mut operators = HashMap::new();
        operators.insert("+", Operator::binary(&|a, b| Ok(a + b)));
        operators.insert("-", Operator::binary(&|a, b| Ok(a - b)));
        operators.insert("*", Operator::binary(&|a, b| Ok(a * b)));
        operators.insert("/", Operator::binary(&|a, b| Ok(a / b)));
        Self { operators }
    }

    pub fn with_operators(
        custom: &'a HashMap<&'a str, Operator<'a>>,
    ) -> Result<Self, &'static str> {
        let mut operators = HashMap::new();
        // TODO: re-use defaults.
        operators.insert("+", Operator::binary(&|a, b| Ok(a + b)));
        operators.insert("-", Operator::binary(&|a, b| Ok(a - b)));
        operators.insert("*", Operator::binary(&|a, b| Ok(a * b)));
        operators.insert("/", Operator::binary(&|a, b| Ok(a / b)));

        for (name, operator) in custom.iter() {
            if let Ok(_) = name.parse::<f64>() {
                return Err("Invalid operator identifier");
            }
            operators.insert(*name, Operator::from(&operator));
        }
        Ok(Self { operators })
    }

    pub fn evaluate(&self, tokens: &[&str]) -> Result<f64, &'static str> {
        let mut stack: Vec<f64> = vec![];
        for token in tokens {
            if let Ok(operand) = token.parse::<f64>() {
                stack.push(operand);
            } else {
                if stack.len() < 2 {
                    return Err("Too few operands");
                }
                let result = match self.operators.get(*token) {
                    Some(operator) => match *operator {
                        Operator::Binary(f) => {
                            if stack.len() < 2 {
                                return Err("Too few operands");
                            }
                            let op2 = stack.pop().unwrap();
                            let op1 = stack.pop().unwrap();
                            (*f)(op1, op2)
                        }
                        Operator::Unary(f) => {
                            if stack.len() < 1 {
                                return Err("Too few operands");
                            }
                            let op = stack.pop().unwrap();
                            (*f)(op)
                        }
                    },
                    _ => return Err("Unrecognized operator"),
                };
                match result {
                    Ok(result) => stack.push(result),
                    Err(e) => return Err(e),
                }
            }
        }

        if stack.len() != 1 {
            Err("Too many operands!")
        } else {
            Ok(stack.pop().unwrap())
        }
    }
}
