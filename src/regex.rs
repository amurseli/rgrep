use std::{any::Any, char, collections::VecDeque, os::unix::process, usize::MAX};

#[derive(Debug, Clone)]
enum Class {
    Alnum,
    Alpha,
    Digit,
    Lower,
    Upper,
    Space,
    Punct,
}

struct EvaluatedStep {
    step: RegexStep,
    size: usize,
    backtrackeable: bool,
}

#[derive(Debug, Clone)]
enum RegexVal {
    Literal(char),
    Wildcard,
    Bracket(Vec<char>),
    NegatedBracket(Vec<char>),
    Class(Class),
}

impl RegexVal {
    fn match_class(value: char, class: &Class) -> usize {
        match class {
            Class::Alnum => {
                if value.is_alphanumeric() {
                    //println!("El caracter {:?} coincidio",value);
                    value.len_utf8()
                } else {
                    //println!("El caracter {:?} NO coincidio",value);
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
            _ => 0,
        }
    }

    pub fn matches(&self, value: &str) -> usize {
        match self {
            RegexVal::Literal(l) => {
                if value.chars().next() == Some(*l) {
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

#[derive(Debug, Clone)]
pub struct RegexStep {
    val: RegexVal,
    rep: RegexRep,
}

#[derive(Debug, Clone)]
enum RegexRep {
    Any,
    Exact(usize),
    Range {
        min: Option<usize>,
        max: Option<usize>,
    },
}

pub struct Regex {
    steps: Vec<RegexStep>,
}

impl Regex {
    fn handle_brackets(char_iter: &mut std::str::Chars) -> Result<RegexStep, &'static str> {
        let mut chars = Vec::new();
        let mut negate = false;
        let mut closed = false;
        let mut is_class = false;
        let mut val = RegexVal::Literal(' ');

        while let Some(ch) = char_iter.next() {
            match ch {
                ']' => {
                    closed = true;
                    break;
                }
                '^' => negate = true,
                '[' => {
                    if (char_iter.next() == Some(':')) {
                        let mut class_name = String::new();
                        while let Some(name_ch) = char_iter.next() {
                            if name_ch == ':' {
                                is_class = true;
                                break;
                            }
                            class_name.push(name_ch);
                        }
                        val = match class_name.as_str() {
                            "alnum" => RegexVal::Class(Class::Alnum),
                            "alpha" => RegexVal::Class(Class::Alpha),
                            "digit" => RegexVal::Class(Class::Digit),
                            "lower" => RegexVal::Class(Class::Lower),
                            "upper" => RegexVal::Class(Class::Upper),
                            "space" => RegexVal::Class(Class::Space),
                            "punct" => RegexVal::Class(Class::Punct),
                            _ => return Err("Clase de caracteres desconocida"),
                        };
                        char_iter.next();
                    }
                }
                _ => chars.push(ch),
            }
        }

        if !closed {
            return Err("No closing bracket found");
        }
        if !is_class {
            val = if negate {
                RegexVal::NegatedBracket(chars)
            } else {
                RegexVal::Bracket(chars)
            };
        }
        Ok(RegexStep {
            rep: RegexRep::Exact(1),
            val,
        })
    }

    fn handle_curly(
        steps: &mut Vec<RegexStep>,
        char_iter: &mut std::str::Chars,
    ) -> Option<RegexStep> {
        let mut min = None;
        let mut max = None;
        let mut num_str = String::new();
        let mut after_comma = false;
        let mut no_comma = true;
        while let Some(ch) = char_iter.next() {
            match ch {
                ',' => {
                    no_comma = false;
                    if let Ok(num) = num_str.trim().parse::<usize>() {
                        if !after_comma {
                            min = Some(num);
                        } else {
                            max = Some(num);
                            break;
                        }
                    } else {
                        min = Some(0);
                    }
                    num_str.clear();
                    after_comma = true;
                }
                '}' => {
                    if let Ok(num) = num_str.trim().parse::<usize>() {
                        if !after_comma {
                            min = Some(num);
                        } else {
                            max = Some(num);
                        }
                        if no_comma {
                            min = Some(num);
                            max = Some(num);
                        }
                    } else {
                        max = Some(MAX);
                    }
                    break;
                }
                '0'..='9' => num_str.push(ch),
                _ => {
                    return Some(RegexStep {
                        rep: RegexRep::Range {
                            min: None,
                            max: None,
                        },
                        val: RegexVal::Literal('{'), //placeholder
                    });
                }
            }
        }

        if let Some(last) = steps.last_mut() {
            last.rep = RegexRep::Range { min, max };
        } else {
            return Some(RegexStep {
                rep: RegexRep::Range {
                    min: None,
                    max: None,
                },
                val: RegexVal::Literal('{'),
            });
        }

        None
    }

    
    pub fn new(exp: &str) -> Result<Vec<Self>, &str> {
        let mut regex_list: Vec<Self> = Vec::new();

        let mut expressions: Vec<&str> = exp.split('|').collect();

        for expression in expressions {
            let mut escaped = false;
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
                    },
                    '\\' => {
                        match handle_backslash(&mut char_iter) {
                            Ok(step) => {
                                if let Some(step) = step {
                                    Some(step)
                                } else {
                                    None
                                }
                            },
                            Err(err) => return Err(err),
                        }
                    }
                    
                    ' ' | '$' | '^' | '(' | ')' | '"' | '!' | ','  =>Some(RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexVal::Literal(c),
                    }),
                    '{' => Self::handle_curly(&mut steps, &mut char_iter),

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
                    
                    '[' => match Self::handle_brackets(&mut char_iter) {
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

    fn process_step(&mut self, mut queue: &mut VecDeque<RegexStep>, value: &str) -> bool {
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
                RegexRep::Exact(n) => {
                    let mut match_size = 0;
                    for _ in 0..=0 {
                        let size = step.val.matches(&value[index..]);
                        if size == 0 {
                            if (!anchored_start) {
                                match backtrack(&step, &mut stack, &mut queue) {
                                    Some(size) => {
                                        index -= size;
                                        continue 'steps;
                                    }
                                    None => {
                                        if (value.len() < index + 1) {
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
                                                match backtrack(&step, &mut stack, &mut queue) {
                                                    Some(size) => {
                                                        index -= size;
                                                        continue 'steps;
                                                    }
                                                    None => {
                                                        if (value.len() < index + 1) {
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

fn handle_backslash(char_iter: &mut std::str::Chars) -> Result<Option<RegexStep>, &'static str> {
    let mut peekable_iter = char_iter.peekable();
    if let Some(&next) = peekable_iter.peek() {
        peekable_iter.next(); 
        // Avanzar el iterador después de procesar el backslash
        if let Some(&next_next) = peekable_iter.peek() {
            peekable_iter.next();
        }
        Ok(Some(RegexStep {
            rep: RegexRep::Exact(1),
            val: RegexVal::Literal(next),
        }))
    } else {
        Err("'\\' inesperado")
    }
}


fn check_min_max(min: Option<usize>, max: Option<usize>, counter: usize) -> bool {
    match min {
        Some(min) => match max {
            Some(max) => {
                if (counter < min || counter > max) {
                    return false;
                } else {
                    return true;
                }
            }
            None => {
                if (counter > min) {
                    return true;
                }
                return false;
            }
        },
        None => {
            return false;
        }
    }
}

fn backtrack(
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
