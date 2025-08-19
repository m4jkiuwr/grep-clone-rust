use crate::pattern::states::{EndAnchor, StartAnchor};

use super::automata::*;
use super::{AlphaNum, Digit, Literal, NegativeGroup, PositiveGroup};

impl<'a> From<&str> for ReAutomata<'a> {
    fn from(value: &str) -> ReAutomata<'a> {
        let mut states = vec![];
        let mut reference: ReRef = 0;
        let mut it = value.chars().peekable();
        while let Some(x) = it.next() {
            let next_elem: Box<ReState> = match x {
                '\\' => match it.peek() {
                    Some('w') => {
                        it.next();
                        Box::new(AlphaNum::new(reference, reference + 1))
                    }
                    Some('d') => {
                        it.next();
                        Box::new(Digit::new(reference, reference + 1))
                    }
                    _ => Box::new(Literal::new('\\', reference, reference + 1)),
                },
                '[' => {
                    let mut group: Vec<char> = vec![];
                    loop {
                        match it.next() {
                            Some(']') => break,
                            Some(c) => group.push(c),
                            None => panic!("Received unclosed group, {:?}", group),
                        }
                    }
                    if group.is_empty() {
                        panic!("Received empty positive group");
                    }
                    if group[0] == '^' {
                        Box::new(NegativeGroup::new(
                            group.into_iter().skip(1),
                            reference,
                            reference + 1,
                        ))
                    } else {
                        Box::new(PositiveGroup::new(
                            group.into_iter(),
                            reference,
                            reference + 1,
                        ))
                    }
                }
                '^' => Box::new(StartAnchor::new(reference, reference + 1)),
                '$' => Box::new(EndAnchor::new(reference, reference + 1)),
                _ => Box::new(Literal::new(x, reference, reference + 1)),
            };
            states.push(next_elem);
            reference += 1;
        }
        ReAutomata::new(states)
    }
}
