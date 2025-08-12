use super::PatternElems;

#[derive(Debug)]
pub struct Numeric;

impl PatternElems for Numeric {
    fn matcher(self) -> Box<dyn Fn(&mut std::str::Chars) -> bool> {
        Box::new(|it| {
            if let Some(c) = it.next() {
                c.is_numeric()
            } else {
                false
            }
        })
    }
}

#[derive(Debug)]
pub struct AlphaNumeric;

impl PatternElems for AlphaNumeric {
    fn matcher(self) -> Box<dyn Fn(&mut std::str::Chars) -> bool> {
        Box::new(|it| {
            if let Some(c) = it.next() {
                c.is_alphanumeric() || c == '_'
            } else {
                false
            }
        })
    }
}

#[derive(Debug)]
pub struct Literal(pub char);

impl PatternElems for Literal {
    fn matcher(self) -> Box<dyn Fn(&mut std::str::Chars) -> bool> {
        Box::new(move |it| it.next() == Some(self.0))
    }
}
