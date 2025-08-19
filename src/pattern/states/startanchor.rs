use super::*;

pub struct StartAnchor {
    reference: ReRef,
    next: ReRef,
}
impl<'a> State<ReInput<'a>, ReRef> for StartAnchor {
    fn reference(&self) -> ReRef {
        self.reference
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<(ReRef, ReInput<'a>)> {
        let mut next_states = vec![];

        if input.offset() == 0 {
            next_states.push((self.next, input.generate(0)));
        }
        if input.current_char() == Some(&'\n') {
            next_states.push((self.next, input.generate(1)));
        }
        next_states
    }
}

impl StartAnchor {
    pub fn new(reference: ReRef, next: ReRef) -> Self {
        Self { reference, next }
    }
}
