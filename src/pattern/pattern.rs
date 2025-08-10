use super::PatternElems;

use super::{classes::*, groups::*};

use std::str::Chars;

pub struct Pattern {
    content: Vec<Box<dyn Fn(&mut Chars) -> bool>>,
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        let mut it = value.chars().peekable();
        let mut content = vec![];
        while let Some(x) = it.next() {
            let next_elem = match x {
                '[' => {
                    let positive = match it.peek() {
                        Some('^') => {
                            it.next();
                            false
                        }
                        Some(']') => panic!("Received empty group"),
                        None => panic!("Group didnt close"),
                        _ => true,
                    };

                    let content: Vec<char> = it.by_ref().take_while(|&c| c != ']').collect();
                    it.next();

                    if positive {
                        let group: PositiveGroup = content.into();
                        group.matcher()
                    } else {
                        let group: NegativeGroup = content.into();
                        group.matcher()
                    }
                }
                '\\' => {
                    if let Some(c) = it.next() {
                        match c {
                            'd' => Numeric.matcher(),
                            'w' => AlphaNumeric.matcher(),
                            _ => todo!(),
                        }
                    } else {
                        panic!("Found '\' on the end of the input")
                    }
                }
                x => Literal(x).matcher(),
            };
            content.push(next_elem)
        }
        Self { content }
    }
}

impl Pattern {
    pub fn inside(&self, text: &str) -> bool {
        let mut it = text.chars();
        self.content.iter().all(|f| f(&mut it))
    }
}
