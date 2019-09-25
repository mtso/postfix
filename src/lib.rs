mod operator;
mod postfix_calculator;

pub use self::operator::Operator;
pub use self::postfix_calculator::PostfixCalculator;

#[cfg(test)]
mod tests {
    use super::Operator;
    use super::PostfixCalculator;
    use std::collections::HashMap;
    use std::f64::INFINITY;

    #[test]
    fn it_adds() {
        let calculator = PostfixCalculator::new();
        let expression = vec!["4", "2", "+"];
        assert_eq!(Ok(6.0), calculator.evaluate(&expression));
    }

    #[test]
    fn it_adds_3() {
        let calculator = PostfixCalculator::new();
        let expression = vec!["1", "2", "3", "+", "+"];
        assert_eq!(Ok(6.0), calculator.evaluate(&expression));
    }

    #[test]
    fn it_subtracts() {
        let calculator = PostfixCalculator::new();
        let expression = vec!["4", "2", "-"];
        assert_eq!(Ok(2.0), calculator.evaluate(&expression));
    }

    #[test]
    fn it_multiplies() {
        let calculator = PostfixCalculator::new();
        let expression = vec!["4", "2", "*"];
        assert_eq!(Ok(8.0), calculator.evaluate(&expression));
    }

    #[test]
    fn it_divides() {
        let calculator = PostfixCalculator::new();
        let expression = vec!["4", "2", "/"];
        assert_eq!(Ok(2.0), calculator.evaluate(&expression));
    }

    #[test]
    fn it_divides_by_zero() {
        let calculator = PostfixCalculator::new();
        let expression = vec!["4", "0", "/"];
        assert_eq!(Ok(INFINITY), calculator.evaluate(&expression));
    }

    #[test]
    fn it_respects_precedence() {
        let calculator = PostfixCalculator::new();
        let expression = vec!["1", "2", "4", "+", "-"];
        assert_eq!(Ok(-5.0), calculator.evaluate(&expression));
    }

    #[test]
    fn it_uses_custom_operators() {
        let mut custom = HashMap::new();
        custom.insert("pow", Operator::binary(&|a: f64, b| Ok(a.powf(b))));
        let calculator = PostfixCalculator::with_operators(&custom).unwrap();
        let expression = vec!["1", "2", "4", "+", "-"];
        assert_eq!(Ok(-5.0), calculator.evaluate(&expression));
    }
}
