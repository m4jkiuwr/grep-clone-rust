use super::*;

#[derive(Debug)]
pub struct EndAnchor {
    reference: ReRef,
    next: ReRef,
}
impl<'a> State<ReInput<'a>, ReRef, ReHash> for EndAnchor {
    fn reference(&self) -> ReRef {
        self.reference
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<ReHash> {
        let curr_char = input.current_char();
        if curr_char == Some(&'\n') || curr_char == None {
            vec![input.to_hash(1, self.next)]
        } else {
            vec![]
        }
    }
    fn next(&self) -> ReRef {
        self.next
    }
}

impl EndAnchor {
    pub fn new(reference: ReRef, next: ReRef) -> Self {
        Self { reference, next }
    }
}
