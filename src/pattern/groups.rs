use super::{Matcher, PatternElems};
use std::collections::HashSet;

#[derive(Debug)]
pub struct PositiveGroup(HashSet<char>);

impl Into<PositiveGroup> for Vec<char> {
    fn into(self) -> PositiveGroup {
        let mut map = HashSet::new();
        self.iter().for_each(|c| {
            map.insert(*c);
        });
        PositiveGroup(map)
    }
}

impl PatternElems for PositiveGroup {
    fn matcher(self) -> Matcher {
        Box::new(move |it, _offset| {
            if let Some(c) = it.next() {
                self.0.contains(&c)
            } else {
                false
            }
        })
    }
}

#[derive(Debug)]
pub struct NegativeGroup(HashSet<char>);

impl Into<NegativeGroup> for Vec<char> {
    fn into(self) -> NegativeGroup {
        let mut map = HashSet::new();
        self.iter().for_each(|c| {
            map.insert(*c);
        });
        NegativeGroup(map)
    }
}

impl PatternElems for NegativeGroup {
    fn matcher(self) -> Matcher {
        Box::new(move |it, _offset| {
            if let Some(c) = it.next() {
                !self.0.contains(&c)
            } else {
                false
            }
        })
    }
}
