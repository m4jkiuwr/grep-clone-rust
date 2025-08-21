use super::*;
#[derive(Debug)]
pub struct Wildcard {
    reference: ReRef,
    next: ReRef,
}
impl<'a> State<ReInput<'a>, ReRef, ReHash> for Wildcard {
    fn reference(&self) -> ReRef {
        self.reference
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<ReHash> {
        match input.current_char() {
            Some(_) => vec![input.to_hash(1, self.next)],
            _ => vec![],
        }
    }
    fn next(&self) -> ReRef {
        self.next
    }
}

impl Wildcard {
    pub fn new(reference: ReRef, next: ReRef) -> Self {
        Self { reference, next }
    }
}
