use std::{collections::HashMap, hash::Hash};

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Array(Vec<Pattern>),
    Literal(char),
    Numeric,
    AlphaNumeric,
    PositiveGroup(Vec<Pattern>),
    NegativeGroup(Vec<Pattern>),
}

impl From<char> for Pattern {
    fn from(value: char) -> Self {
        match value {
            'd' => Self::Numeric,
            'w' => Self::AlphaNumeric,
            _ => todo!(),
        }
    }
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        let mut it = value.chars().peekable();
        let mut content: Vec<Pattern> = vec![];
        while let Some(x) = it.next() {
            content.push(match x {
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

                    let content = it
                        .by_ref()
                        .take_while(|&c| c != ']')
                        .map(|c| Pattern::Literal(c))
                        .collect();
                    it.next();

                    if positive {
                        Pattern::PositiveGroup(content)
                    } else {
                        Pattern::NegativeGroup(content)
                    }
                }
                '\\' => {
                    if let Some(c) = it.next() {
                        Pattern::from(c)
                    } else {
                        panic!("Found '\' on the end of the input")
                    }
                }
                x => Self::Literal(x),
            });
        }
        Pattern::Array(content)
    }
}

impl Pattern {
    pub fn inside(&self, text: &str) -> bool {
        match self {
            Pattern::Array(x) => {
                if x.len() != 1 {
                    todo!()
                } else {
                    x[0].inside(text)
                }
            }
            Pattern::Literal(c) => text.contains(*c),
            Pattern::Numeric => text.chars().any(|x| x.is_numeric()),
            Pattern::AlphaNumeric => text.chars().any(|x| x.is_alphanumeric() || x == '_'),
            Pattern::PositiveGroup(_) => self.positive_group_inside(text),
            Pattern::NegativeGroup(_) => self.negative_group_inside(text),

            _ => todo!(),
        }
    }
}

impl Pattern {
    fn positive_group_inside(&self, text: &str) -> bool {
        if let Pattern::PositiveGroup(content) = self {
            let mut map: HashMap<char, bool> = HashMap::new();
            content.iter().for_each(|x| {
                match x {
                    Pattern::Literal(c) => map.insert(*c, true),
                    _ => todo!(),
                };
            });
            text.chars().any(|c| map.contains_key(&c))
        } else {
            panic!("Used positive_group_inside for wrong pattern : {:?}", self)
        }
    }
    fn negative_group_inside(&self, text: &str) -> bool {
        if let Pattern::NegativeGroup(content) = self {
            let mut map: HashMap<char, bool> = HashMap::new();
            content.iter().for_each(|x| {
                match x {
                    Pattern::Literal(c) => map.insert(*c, true),
                    _ => todo!(),
                };
            });
            text.chars().any(|c| !map.contains_key(&c))
        } else {
            panic!("Used positive_group_inside for wrong pattern : {:?}", self)
        }
    }
}
