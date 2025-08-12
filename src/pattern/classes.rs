use super::{Matcher, PatternElems};

#[derive(Debug)]
pub struct Numeric;

impl PatternElems for Numeric {
    fn matcher(self) -> Matcher {
        Box::new(|it, _offset| {
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
    fn matcher(self) -> Matcher {
        Box::new(|it, _offset| {
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
    fn matcher(self) -> Matcher {
        Box::new(move |it, _offset| it.next() == Some(self.0))
    }
}
