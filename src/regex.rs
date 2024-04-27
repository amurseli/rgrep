use crate::class::Class;
use crate::evaluated_step::EvaluatedStep;
use crate::regex_rep::RegexRep;
use crate::regex_step::{Regex, RegexStep};
use crate::regex_val::RegexVal;
use crate::utils::{check_min_max, handle_backslash, handle_brackets, handle_curly};
use std::{char, collections::VecDeque, usize::MAX};

impl RegexVal {
    /// Matches a character against a specified character class and returns the length of the match.
    ///
    /// # Arguments
    ///
    /// * `value` - The character to match.
    /// * `class` - The character class to match against.
    ///
    /// # Returns
    ///
    /// Returns the length of the match if the character matches the specified class, otherwise returns 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::structures::Class;
    /// let value = 'A';
    /// let class = Class::Upper;
    /// let length = match_class(value, &class);
    /// ```
    fn match_class(value: char, class: &Class) -> usize {
        match class {
            Class::Alnum => {
                if value.is_alphanumeric() {
                    value.len_utf8()
                } else {
                    0
                }
            }
            Class::Alpha => {
                if value.is_alphabetic() {
                    value.len_utf8()
                } else {
                    0
                }
            }
            Class::Digit => {
                if value.is_ascii_digit() {
                    value.len_utf8()
                } else {
                    0
                }
            }
            Class::Lower => {
                if value.is_lowercase() {
                    value.len_utf8()
                } else {
                    0
                }
            }
            Class::Upper => {
                if value.is_uppercase() {
                    value.len_utf8()
                } else {
                    0
                }
            }
            Class::Space => {
                if value.is_whitespace() {
                    value.to_string().len()
                } else {
                    0
                }
            }
            Class::Punct => {
                if value.is_ascii_punctuation() {
                    value.to_string().len()
                } else {
                    0
                }
            }
        }
    }

    /// Matches a slice against the regular expression value and returns its length.
    ///
    /// # Arguments
    ///
    /// * `value` - A slice to match against the regular expression.
    ///
    /// # Returns
    ///
    /// The length of the match if the value matches the regular expression, otherwise 0.

    pub fn matches(&self, value: &str) -> usize {
        match self {
            RegexVal::Literal(l) => {
                if value.starts_with(*l) {
                    l.len_utf8()
                } else {
                    0
                }
            }
            RegexVal::Wildcard => {
                if let Some(w) = value.chars().next() {
                    w.len_utf8()
                } else {
                    0
                }
            }
            RegexVal::Bracket(chars) => {
                if let Some(c) = value.chars().next() {
                    if chars.contains(&c) {
                        c.len_utf8()
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            RegexVal::NegatedBracket(chars) => {
                if let Some(c) = value.chars().next() {
                    if chars.contains(&c) {
                        0
                    } else {
                        c.len_utf8()
                    }
                } else {
                    0
                }
            }
            RegexVal::Class(class) => {
                if let Some(c) = value.chars().next() {
                    Self::match_class(c, class)
                } else {
                    0
                }
            }
        }
    }
}

impl Regex {
    pub fn new(exp: &str) -> Result<Vec<Self>, &str> {
        let mut regex_list: Vec<Self> = Vec::new();

        let expressions: Vec<&str> = exp.split('|').collect();

        for expression in expressions {
            let mut steps: Vec<RegexStep> = Vec::new();
            let mut char_iter = expression.chars();
            while let Some(c) = char_iter.next() {
                let step = match c {
                    '.' => Some(RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexVal::Wildcard,
                    }),
                    'a'..='z' => Some(RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexVal::Literal(c),
                    }),
                    '*' => {
                        if let Some(last) = steps.last_mut() {
                            last.rep = RegexRep::Any;
                        } else {
                            return Err("'*' Inesperado");
                        }
                        None
                    }
                    '+' => {
                        if let Some(last) = steps.last_mut() {
                            last.rep = RegexRep::Range {
                                min: Some(2),
                                max: Some(MAX),
                            };
                        } else {
                            return Err("'+' Inesperado");
                        }
                        None
                    }
                    '\\' => match handle_backslash(&mut char_iter) {
                        Ok(step) => step,
                        Err(err) => return Err(err),
                    },

                    ' ' | '$' | '^' | '(' | ')' | '"' | '!' | ',' | ':' | '-' => Some(RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexVal::Literal(c),
                    }),
                    '{' => handle_curly(&mut steps, &mut char_iter),

                    '?' => {
                        if let Some(last) = steps.last_mut() {
                            last.rep = RegexRep::Range {
                                min: Some(1),
                                max: Some(2),
                            }; //por como lo hice, esto representa 0 repeticiones a 1
                        } else {
                            return Err("'+' Inesperado");
                        }
                        None
                    }

                    '[' => match handle_brackets(&mut char_iter) {
                        Ok(step) => Some(step),
                        Err(err) => return Err(err),
                    },

                    _ => return Err("Caracter Inesperado"),
                };
                if let Some(p) = step {
                    steps.push(p);
                }
            }
            regex_list.push(Regex { steps })
        }

        Ok(regex_list)
    }

    pub fn test(&mut self, value: &str) -> Result<String, &str> {
        if !value.is_ascii() {
            return Err("El input no es ascii");
        }

        let mut queue = VecDeque::new();
        for step in self.steps.drain(..) {
            queue.push_back(step);
        }

        if self.process_step(&mut queue, value) {
            return Ok(value.to_string());
        }

        Ok("".to_string())
    }

    fn process_step(&mut self, queue: &mut VecDeque<RegexStep>, value: &str) -> bool {
        let mut stack: Vec<EvaluatedStep> = Vec::new();
        let mut index = 0;
        let mut anchored_start = false;
        let mut anchored_end = false;

        if let Some(first_step) = queue.front() {
            if let RegexVal::Literal('^') = first_step.val {
                anchored_start = true;
                queue.pop_front();
            }
        }
        if let Some(first_step) = queue.back() {
            if let RegexVal::Literal('$') = first_step.val {
                anchored_end = true;
                queue.pop_back();
            }
        }

        'steps: while let Some(step) = queue.pop_front() {
            match step.rep {
                RegexRep::Exact(_) => {
                    let mut match_size = 0;
                    for _ in 0..=0 {
                        let size = step.val.matches(&value[index..]);
                        if size == 0 {
                            if !anchored_start {
                                match backtrack(&step, &mut stack, queue) {
                                    Some(size) => {
                                        index -= size;
                                        continue 'steps;
                                    }
                                    None => {
                                        if value.len() < index + 1 {
                                            return false;
                                        }
                                        match_size += 1;
                                        index += 1;
                                    }
                                }
                            } else {
                                return false;
                            }
                        } else {
                            match_size += size;
                            index += size;
                            stack.push(EvaluatedStep {
                                step: step.clone(),
                                size: match_size,
                                backtrackeable: false,
                            });
                        }
                    }
                }
                RegexRep::Any => {
                    let mut keep_matching = true;
                    //println!("n {:?}", step.val);
                    while keep_matching {
                        let match_size = step.val.matches(&value[index..]);
                        if match_size != 0 {
                            index += match_size;
                            if let Some(next_step) = queue.front() {
                                if let RegexVal::Literal(next_char) = &next_step.val {
                                    if let Some(current_char) = value.chars().nth(index) {
                                        if current_char == *next_char {
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            keep_matching = false;
                        }
                    }
                }
                RegexRep::Range { min, max } => {
                    let mut keep_matching = true;
                    let mut counter: usize = 0;
                    //println!("n {:?}", step.val);
                    while keep_matching {
                        let match_size = step.val.matches(&value[index..]);
                        if match_size != 0 {
                            counter += 1;
                            index += match_size;
                            if let Some(next_step) = queue.front() {
                                if let RegexVal::Literal(next_char) = &next_step.val {
                                    if let Some(current_char) = value.chars().nth(index) {
                                        if current_char == *next_char {
                                            if !check_min_max(min, max, counter) {
                                                match backtrack(&step, &mut stack, queue) {
                                                    Some(size) => {
                                                        index -= size;
                                                        continue 'steps;
                                                    }
                                                    None => {
                                                        if value.len() < index + 1 {
                                                            return false;
                                                        }
                                                        index += 1;
                                                    }
                                                }
                                            }

                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            keep_matching = false;
                        }
                    }
                }
            }
        }
        if anchored_end && index != value.len() {
            return false;
        }

        true
    }
}

pub fn backtrack(
    current: &RegexStep,
    evaluated: &mut Vec<EvaluatedStep>,
    next: &mut VecDeque<RegexStep>,
) -> Option<usize> {
    let mut back_size = 0;
    next.push_front(current.clone());
    while let Some(e) = evaluated.pop() {
        back_size += e.size;
        if e.backtrackeable {
            //println!("Backtrack {:?}", back_size);
            return Some(back_size);
        } else {
            next.push_front(e.step);
        }
    }

    None
}
