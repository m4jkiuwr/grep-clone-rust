use super::{Matcher, PatternElems};

use super::{anchors::*, classes::*, groups::*};

const CLASSES: &str = "dw";

pub struct Pattern {
    content: Vec<Matcher>,
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        let mut it = value.chars().peekable();
        let mut content = vec![];
        while let Some(x) = it.next() {
            let next_elem = match x {
                '^' => StartOfString.matcher(),
                '$' => EndOfLine.matcher(),
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
                    if let Some(x) = it.next_if(|a| CLASSES.contains(*a)) {
                        match x {
                            'd' => Numeric.matcher(),
                            'w' => AlphaNumeric.matcher(),
                            _ => unreachable!(),
                        }
                    } else {
                        Literal('\\').matcher()
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
        (0..text.len()).any(|offset| {
            if text.is_char_boundary(offset) {
                let mut it = text[offset..].chars();
                self.content.iter().all(|f| f(&mut it, offset))
            } else {
                false
            }
        })
    }
}
