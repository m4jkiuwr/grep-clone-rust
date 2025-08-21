use super::*;

#[derive(Debug)]
pub struct Literal {
    literal: char,
    reference: ReRef,
    next: ReRef,
}
impl<'a> State<ReInput<'a>, ReRef, ReHash> for Literal {
    fn reference(&self) -> ReRef {
        self.reference
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<ReHash> {
        match input.current_char() {
            Some(x) if *x == self.literal => vec![input.to_hash(1, self.next)],
            _ => vec![],
        }
    }
    fn next(&self) -> ReRef {
        self.next
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
