use super::*;
use std::collections::HashSet;

pub struct PositiveGroup {
    group: HashSet<char>,
    reference: ReRef,
    next: ReRef,
}
impl<'a> State<ReInput<'a>, ReRef> for PositiveGroup {
    fn reference(&self) -> ReRef {
        self.reference
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<(ReRef, ReInput<'a>)> {
        match input.current_char() {
            Some(x) if self.group.contains(x) => vec![(self.next, input.generate(1))],
            _ => vec![],
        }
    }
}

impl PositiveGroup {
    pub fn new(group: impl Iterator<Item = char>, reference: ReRef, next: ReRef) -> Self {
        Self {
            group: HashSet::from_iter(group),
            reference,
            next,
        }
    }
}
