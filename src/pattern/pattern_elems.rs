use std::str::Chars;

pub trait PatternElems {
    fn matcher(self) -> Box<dyn Fn(&mut Chars) -> bool>;
}
