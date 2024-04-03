use crate::regex_step::RegexStep;

pub struct EvaluatedStep {
    pub step: RegexStep,
    pub size: usize,
    pub backtrackeable: bool,
}
