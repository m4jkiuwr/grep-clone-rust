pub enum Pattern {
    Array(Vec<Pattern>),
    Literal(char),
    Numeric,
    AlphaNumeric,
    PositiveGroup(Vec<Pattern>),
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
        let mut it = value.chars();
        let mut content: Vec<Pattern> = vec![];
        while let Some(x) = it.next() {
            content.push(match x {
                '[' => {
                    let group = it
                        .by_ref()
                        .take_while(|&c| c != ']')
                        .map(|c| Pattern::Literal(c))
                        .collect();
                    it.next();
                    Pattern::PositiveGroup(group)
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
            Pattern::PositiveGroup(cs) => cs.iter().any(|lit| lit.inside(text)),
            _ => todo!(),
        }
    }
}
