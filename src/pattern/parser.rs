use std::str::Chars;

use crate::pattern::states::{EndAnchor, Group, OrElse, StartAnchor, Wildcard, ZeroOrOne};

use super::automata::*;
use super::{AlphaNum, Digit, Literal, NegativeGroup, OneOrMore, PositiveGroup};

impl<'a> From<&mut Chars<'a>> for ReAutomata<'a> {
    fn from(it: &mut Chars<'a>) -> ReAutomata<'a> {
        let mut states: Vec<Box<ReState<'a>>> = vec![];
        let mut reference: ReRef = 0;
        while let Some(x) = it.next() {
            let next_elem: Box<ReState<'a>> = match x {
                '\\' => match it.next() {
                    Some('w') => {
                        it.next();
                        Box::new(AlphaNum::new(reference, reference + 1))
                    }
                    Some('d') => {
                        it.next();
                        Box::new(Digit::new(reference, reference + 1))
                    }
                    Some(x) => Box::new(Literal::new(x, reference, reference + 1)),
                    None => panic!("Received '\\' at the end of the input"),
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
                '+' => match states.pop() {
                    Some(s) => Box::new(OneOrMore::new(s)),
                    None => panic!("No state provided before '+'"),
                },
                '?' => match states.pop() {
                    Some(s) => Box::new(ZeroOrOne::new(s)),
                    None => panic!("No state provided before '?'"),
                },
                '^' => Box::new(StartAnchor::new(reference, reference + 1)),
                '$' => Box::new(EndAnchor::new(reference, reference + 1)),
                '.' => Box::new(Wildcard::new(reference, reference + 1)),
                ')' => break,
                '(' => Box::new(Group::new(
                    ReAutomata::from(it.by_ref()),
                    reference,
                    reference + 1,
                )),
                '|' => {
                    let lhs = ReAutomata::new(states);
                    let rhs = ReAutomata::from(it.by_ref());
                    states = vec![];

                    Box::new(OrElse::new(lhs, rhs, 0, 1))
                }

                _ => Box::new(Literal::new(x, reference, reference + 1)),
            };
            states.push(next_elem);
            reference = states.len();
        }
        ReAutomata::new(states)
    }
}

mod tests {
    use crate::pattern::ReAutomata;

    #[test]
    fn one_or_more_many() {
        let text = ['a'; 5];
        let mut pat = "a+".chars();
        let mut automata = ReAutomata::from(&mut pat);

        let matches = automata.all_matches(&text);
        assert_eq!(
            matches,
            vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (0, 5),
                (1, 2),
                (1, 3),
                (1, 4),
                (1, 5),
                (2, 3),
                (2, 4),
                (2, 5),
                (3, 4),
                (3, 5),
                (4, 5)
            ]
        )
    }
}
