use super::*;

pub struct Digit {
    reference: ReRef,
    next: ReRef,
}
impl<'a> State<ReInput<'a>, ReRef> for Digit {
    fn reference(&self) -> ReRef {
        self.reference
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<(ReRef, ReInput<'a>)> {
        match input.current_char() {
            Some(x) if x.is_numeric() => vec![(self.next, input.generate(1))],
            _ => vec![],
        }
    }
}

impl Digit {
    pub fn new(reference: ReRef, next: ReRef) -> Self {
        Self { reference, next }
    }
}
