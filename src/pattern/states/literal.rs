use super::*;

pub struct Literal {
    literal: char,
    reference: ReRef,
    next: ReRef,
}
impl<'a> State<ReInput<'a>, ReRef> for Literal {
    fn reference(&self) -> ReRef {
        self.reference
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<(ReRef, ReInput<'a>)> {
        match input.current_char() {
            Some(x) if *x == self.literal => vec![(self.next, input.generate(1))],
            _ => vec![],
        }
    }
}

impl Literal {
    pub fn new(literal: char, reference: ReRef, next: ReRef) -> Self {
        Self {
            literal,
            reference,
            next,
        }
    }
}
