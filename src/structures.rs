#[derive(Debug, Clone)]
pub enum Class {
    Alnum,
    Alpha,
    Digit,
    Lower,
    Upper,
    Space,
    Punct,
}

pub struct EvaluatedStep {
    pub step: RegexStep,
    pub size: usize,
    pub backtrackeable: bool,
}

#[derive(Debug, Clone)]
pub enum RegexVal {
    Literal(char),
    Wildcard,
    Bracket(Vec<char>),
    NegatedBracket(Vec<char>),
    Class(Class),
}

#[derive(Debug, Clone)]
pub struct RegexStep {
    pub val: RegexVal,
    pub rep: RegexRep,
}

#[derive(Debug, Clone)]
pub enum RegexRep {
    Any,
    Exact(usize),
    Range {
        min: Option<usize>,
        max: Option<usize>,
    },
}

pub struct Regex {
    pub steps: Vec<RegexStep>,
}
