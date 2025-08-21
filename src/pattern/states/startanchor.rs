use super::*;
#[derive(Debug)]
pub struct StartAnchor {
    reference: ReRef,
    next: ReRef,
}
impl<'a> State<ReInput<'a>, ReRef, ReHash> for StartAnchor {
    fn reference(&self) -> ReRef {
        self.reference
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<ReHash> {
        let mut next_states = vec![];

        if input.offset() == 0 {
            next_states.push(input.to_hash(0, self.next));
        }
        if input.current_char() == Some(&'\n') {
            next_states.push(input.to_hash(1, self.next));
        }
        next_states
    }
    fn next(&self) -> ReRef {
        self.next
    }
}

impl StartAnchor {
    pub fn new(reference: ReRef, next: ReRef) -> Self {
        Self { reference, next }
    }
}
