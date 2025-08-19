use super::*;

pub struct Digit {
    reference: ReRef,
    next: ReRef,
}
impl<'a> State<ReInput<'a>, ReRef, ReHash> for Digit {
    fn reference(&self) -> ReRef {
        self.reference
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<ReHash> {
        match input.current_char() {
            Some(x) if x.is_numeric() => vec![input.to_hash(1, self.next)],
            _ => vec![],
        }
    }
}

impl Digit {
    pub fn new(reference: ReRef, next: ReRef) -> Self {
        Self { reference, next }
    }
}
