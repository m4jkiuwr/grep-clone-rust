pub trait State<Input, StateRef> {
    fn transition(&self, input: Input) -> Vec<(StateRef, Input)>;
    fn reference(&self) -> StateRef;
}

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
    pub fn generate(&self, consumed: usize) -> Self {
        Self {
            offset: self.offset,
            curr_indice: self.curr_indice + consumed,
            text: self.text,
        }
    }
    pub fn offset(&self) -> usize {
        self.offset
    }
}

pub type ReOutput = bool;
pub type ReRef = usize;

pub type ReState<'a> = dyn State<ReInput<'a>, ReRef>;

pub struct ReAutomata<'a> {
    states: Vec<Box<ReState<'a>>>,
    start_state_ref: ReRef,
    end_state_ref: ReRef,
}

impl<'a> ReAutomata<'a> {
    fn get_state(&self, state_ref: ReRef) -> &ReState<'a> {
        self.states.get(state_ref).unwrap().as_ref()
    }
}

impl<'a> ReAutomata<'a> {
    fn single_match(&self, text: &'a [char], offset: usize) -> ReOutput {
        let mut state_stack: Vec<(ReRef, ReInput)> =
            vec![(self.start_state_ref, ReInput::new(text, offset, offset))];
        loop {
            let (state_ref, input) = state_stack.pop().unwrap();
            if state_ref == self.end_state_ref {
                break true;
            }
            let state = self.get_state(state_ref);
            state_stack.extend(state.transition(input));

            if state_stack.is_empty() {
                break false;
            }
        }
    }
    pub fn run(&self, text: &'a [char]) -> ReOutput {
        (0..text.len()).any(|offset| self.single_match(text, offset))
    }
}

impl<'a> ReAutomata<'a> {
    pub fn new(states: Vec<Box<ReState<'a>>>) -> Self {
        let end_state_ref = states.len();
        Self {
            states,
            start_state_ref: 0,
            end_state_ref,
        }
    }
}
