use std::str::Chars;

pub type Matcher = Box<dyn Fn(&mut Chars, usize) -> bool>;
pub trait PatternElems {
    fn matcher(self) -> Matcher;
}
