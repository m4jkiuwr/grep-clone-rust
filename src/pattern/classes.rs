use super::PatternElems;

#[derive(Debug)]
pub struct Numeric;

impl PatternElems for Numeric {
    fn matcher(self) -> Box<dyn Fn(&mut std::str::Chars) -> bool> {
        Box::new(|it| it.any(|c| c.is_numeric()))
    }
}

#[derive(Debug)]
pub struct AlphaNumeric;

impl PatternElems for AlphaNumeric {
    fn matcher(self) -> Box<dyn Fn(&mut std::str::Chars) -> bool> {
        Box::new(|it| it.any(|c| c.is_alphanumeric() || c == '_'))
    }
}

#[derive(Debug)]
pub struct Literal(pub char);

impl PatternElems for Literal {
    fn matcher(self) -> Box<dyn Fn(&mut std::str::Chars) -> bool> {
        Box::new(move |it| it.any(|c| c == self.0))
    }
}
