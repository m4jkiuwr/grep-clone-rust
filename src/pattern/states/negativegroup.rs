use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub struct NegativeGroup {
    group: HashSet<char>,
    reference: ReRef,
    next: ReRef,
}
impl<'a> State<ReInput<'a>, ReRef, ReHash> for NegativeGroup {
    fn reference(&self) -> ReRef {
        self.reference
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<ReHash> {
        match input.current_char() {
            Some(x) if !self.group.contains(x) => vec![input.to_hash(1, self.next)],
            _ => vec![],
        }
    }
    fn next(&self) -> ReRef {
        self.next
    }
}

impl NegativeGroup {
    pub fn new(group: impl Iterator<Item = char>, reference: ReRef, next: ReRef) -> Self {
        Self {
            group: HashSet::from_iter(group),
            reference,
            next,
        }
    }
}
