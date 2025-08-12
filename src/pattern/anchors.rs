use super::{Matcher, PatternElems};

#[derive(Debug)]
pub struct StartOfString;

impl PatternElems for StartOfString {
    fn matcher(self) -> Matcher {
        Box::new(|_it, offset| offset == 0)
    }
}

#[derive(Debug)]
pub struct EndOfLine;

impl PatternElems for EndOfLine {
    fn matcher(self) -> Matcher {
        Box::new(|it, _offset| {
            if let Some(c) = it.next() {
                c == '\n'
            } else {
                true
            }
        })
    }
}
