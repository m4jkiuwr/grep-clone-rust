use std::cell::RefCell;

use super::*;
#[derive(Debug)]
pub struct Group<'a> {
    content: RefCell<ReAutomata<'a>>,
    reference: ReRef,
    next: ReRef,
}
impl<'a> State<ReInput<'a>, ReRef, ReHash> for Group<'a> {
    fn reference(&self) -> ReRef {
        self.reference
    }
    fn next(&self) -> ReRef {
        self.next
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<ReHash> {
        let text = input.text();
        let offset = input.curr_indice();

        self.content
            .borrow_mut()
            .offset_match(text, offset)
            .into_iter()
            .map(|(begin, end)| ReInput::new(text, begin, end).to_hash(0, self.next()))
            .collect()
    }
}

impl<'a> Group<'a> {
    pub fn new(content: ReAutomata<'a>, reference: ReRef, next: ReRef) -> Self {
        Self {
            content: RefCell::new(content),
            reference,
            next,
        }
    }
}
