use std::cell::RefCell;

use super::*;
#[derive(Debug)]
pub struct OrElse<'a> {
    lhs: RefCell<ReAutomata<'a>>,
    rhs: RefCell<ReAutomata<'a>>,
    reference: ReRef,
    next: ReRef,
}
impl<'a> State<ReInput<'a>, ReRef, ReHash> for OrElse<'a> {
    fn reference(&self) -> ReRef {
        self.reference
    }
    fn next(&self) -> ReRef {
        self.next
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<ReHash> {
        let text = input.text();
        let offset = input.curr_indice();

        let lhs_matches = self.lhs.borrow_mut().offset_match(text, offset);
        let rhs_matches = self.rhs.borrow_mut().offset_match(text, offset);

        [lhs_matches, rhs_matches]
            .concat()
            .into_iter()
            .map(|(begin, end)| ReInput::new(text, begin, end).to_hash(0, self.next()))
            .collect()
    }
}

impl<'a> OrElse<'a> {
    pub fn new(lhs: ReAutomata<'a>, rhs: ReAutomata<'a>, reference: ReRef, next: ReRef) -> Self {
        Self {
            lhs: RefCell::new(lhs),
            rhs: RefCell::new(rhs),
            reference,
            next,
        }
    }
}
