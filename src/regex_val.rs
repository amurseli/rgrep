use crate::class::Class;
#[derive(Debug, Clone)]
pub enum RegexVal {
    Literal(char),
    Wildcard,
    Bracket(Vec<char>),
    NegatedBracket(Vec<char>),
    Class(Class),
}
