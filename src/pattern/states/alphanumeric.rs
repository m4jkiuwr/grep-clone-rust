use super::*;

pub struct AlphaNum {
    reference: ReRef,
    next: ReRef,
}
impl<'a> State<ReInput<'a>, ReRef> for AlphaNum {
    fn reference(&self) -> ReRef {
        self.reference
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<(ReRef, ReInput<'a>)> {
        match input.current_char() {
            Some(x) if x.is_alphanumeric() || *x == '_' => vec![(self.next, input.generate(1))],
            _ => vec![],
        }
    }
}

impl AlphaNum {
    pub fn new(reference: ReRef, next: ReRef) -> Self {
        Self { reference, next }
    }
}
