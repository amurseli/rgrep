use crate::regex_rep::RegexRep;
use crate::regex_val::RegexVal;
#[derive(Debug, Clone)]
pub struct RegexStep {
    pub val: RegexVal,
    pub rep: RegexRep,
}

pub struct Regex {
    pub steps: Vec<RegexStep>,
}
