use super::*;
use crate::pattern::automata::ReState;
#[derive(Debug)]
pub struct ZeroOrOne<'a> {
    state: Box<ReState<'a>>,
}
impl<'a> State<ReInput<'a>, ReRef, ReHash> for ZeroOrOne<'a> {
    fn transition(&self, input: ReInput<'a>) -> Vec<ReHash> {
        let skip_state = input.to_hash(0, self.next());
        let mut base_states = self.state.transition(input);

        base_states.push(skip_state);
        base_states
    }
    fn reference(&self) -> ReRef {
        self.state.reference()
    }
    fn next(&self) -> ReRef {
        self.state.next()
    }
}

impl<'a> ZeroOrOne<'a> {
    pub fn new(state: Box<ReState<'a>>) -> Self {
        Self { state }
    }
}
