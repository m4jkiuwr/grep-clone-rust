use super::*;
use crate::pattern::automata::ReState;
#[derive(Debug)]
pub struct OneOrMore<'a> {
    state: Box<ReState<'a>>,
}
impl<'a> State<ReInput<'a>, ReRef, ReHash> for OneOrMore<'a> {
    fn reference(&self) -> ReRef {
        self.state.reference()
    }
    fn transition(&self, input: ReInput<'a>) -> Vec<ReHash> {
        let base_states = self.state.transition(input);
        let looped_states = base_states
            .iter()
            .map(|&hash| {
                let mut new_hash = hash;
                new_hash.set_reference(self.reference());
                new_hash
            })
            .collect();

        [looped_states, base_states].concat()
    }
    fn next(&self) -> ReRef {
        self.state.next()
    }
}

impl<'a> OneOrMore<'a> {
    pub fn new(state: Box<ReState<'a>>) -> Self {
        Self { state }
    }
}
