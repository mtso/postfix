pub enum Operator<'a> {
    Unary(&'a Fn(f64) -> Result<f64, &'static str>),
    Binary(&'a Fn(f64, f64) -> Result<f64, &'static str>),
}

impl<'a> Operator<'a> {
    pub fn from(from: &'a Operator<'a>) -> Self {
        match from {
            Operator::Unary(f) => Operator::unary(f),
            Operator::Binary(f) => Operator::binary(f),
        }
    }

    pub fn unary(f: &'a Fn(f64) -> Result<f64, &'static str>) -> Self {
        Operator::Unary(f)
    }

    pub fn binary(f: &'a Fn(f64, f64) -> Result<f64, &'static str>) -> Self {
        Operator::Binary(f)
    }
}
