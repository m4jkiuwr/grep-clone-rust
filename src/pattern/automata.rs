use std::collections::HashSet;

pub trait State<Input, StateRef, StateHash> {
    fn transition(&self, input: Input) -> Vec<StateHash>;
    fn reference(&self) -> StateRef;
    fn next(&self) -> StateRef;
}

#[derive(Debug, Clone, Copy)]
pub struct ReInput<'a> {
    offset: usize,
    curr_indice: usize,
    text: &'a [char],
}
impl<'a> ReInput<'a> {
    pub fn new(text: &'a [char], offset: usize, curr_indice: usize) -> Self {
        Self {
            offset,
            curr_indice,
            text,
        }
    }
    pub fn current_char(&self) -> Option<&char> {
        self.text.get(self.curr_indice)
    }
    pub fn offset(&self) -> usize {
        self.offset
    }
    pub fn text(&self) -> &'a [char] {
        self.text
    }
    pub fn curr_indice(&self) -> usize {
        self.curr_indice
    }

    pub fn to_hash(&self, consumed: usize, next_state_ref: ReRef) -> ReHash {
        ReHash {
            offset: self.offset,
            curr_indice: self.curr_indice + consumed,
            state_reference: next_state_ref,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ReHash {
    offset: usize,
    curr_indice: usize,
    state_reference: ReRef,
}
impl ReHash {
    fn to_input<'a>(&self, text: &'a [char]) -> ReInput<'a> {
        ReInput::new(text, self.offset, self.curr_indice)
    }
    pub fn set_reference(&mut self, new_ref: ReRef) {
        self.state_reference = new_ref;
    }
    pub(crate) fn to_match(&self) -> ReMatch {
        (self.offset, self.curr_indice)
    }
}

pub type ReMatch = (usize, usize);
pub type ReOutput = Vec<ReMatch>;
pub type ReRef = usize;

pub type ReState<'a> = dyn State<ReInput<'a>, ReRef, ReHash> + 'a;

pub struct ReAutomata<'a> {
    states: Vec<Box<ReState<'a>>>,
    visited: HashSet<ReHash>,
    start_state_ref: ReRef,
    end_state_ref: ReRef,
}

impl<'a> ReAutomata<'a> {
    fn get_state(&self, state_ref: ReRef) -> &ReState<'a> {
        self.states.get(state_ref).unwrap().as_ref()
    }
}

impl<'a> ReAutomata<'a> {
    pub fn offset_match(&mut self, text: &'a [char], offset: usize) -> ReOutput {
        self.visited = HashSet::new();

        let init_input = ReInput::new(text, offset, offset);
        let mut state_stack: Vec<ReHash> = vec![init_input.to_hash(0, self.start_state_ref)];
        let mut matches: ReOutput = vec![];

        loop {
            let hash = state_stack.pop().unwrap();
            if self.visited.contains(&hash) {
                continue;
            }
            let state_ref: ReRef = hash.state_reference;
            let input: ReInput<'a> = hash.to_input(text);

            if state_ref == self.end_state_ref {
                matches.push(hash.to_match());
            } else {
                state_stack.extend(self.get_state(state_ref).transition(input));
                self.visited.insert(hash);
            }
            if state_stack.is_empty() {
                break;
            }
        }
        matches
    }

    pub fn all_matches(&mut self, text: &'a [char]) -> ReOutput {
        (0..text.len())
            .flat_map(|offset| self.offset_match(text, offset))
            .collect()
    }
}

impl<'a> ReAutomata<'a> {
    pub fn new(states: Vec<Box<ReState<'a>>>) -> Self {
        let end_state_ref = states.len();
        Self {
            states,
            visited: HashSet::new(),
            start_state_ref: 0,
            end_state_ref,
        }
    }
}
