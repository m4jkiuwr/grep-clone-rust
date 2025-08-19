use super::*;

pub struct AlphaNum {
    reference: ReRef,
    next: ReRef,
}
impl<'a> State<ReInput<'a>, ReRef, ReHash> for AlphaNum {
    fn reference(&self) -> ReRef {
        self.reference
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<ReHash> {
        match input.current_char() {
            Some(x) if x.is_alphanumeric() || *x == '_' => vec![input.to_hash(1, self.next)],
            _ => vec![],
        }
    }
    fn next(&self) -> ReRef {
        self.next
    }
}

impl AlphaNum {
    pub fn new(reference: ReRef, next: ReRef) -> Self {
        Self { reference, next }
    }
}
