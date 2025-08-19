use super::*;

pub struct EndAnchor {
    reference: ReRef,
    next: ReRef,
}
impl<'a> State<ReInput<'a>, ReRef> for EndAnchor {
    fn reference(&self) -> ReRef {
        self.reference
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<(ReRef, ReInput<'a>)> {
        let curr_char = input.current_char();
        if curr_char == Some(&'\n') || curr_char == None {
            vec![(self.next, input.generate(1))]
        } else {
            vec![]
        }
    }
}

impl EndAnchor {
    pub fn new(reference: ReRef, next: ReRef) -> Self {
        Self { reference, next }
    }
}
