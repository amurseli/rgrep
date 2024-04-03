use crate::class::Class;
use crate::regex_rep::RegexRep;
use crate::regex_step::RegexStep;
use crate::regex_val::RegexVal;
use std::usize::MAX;

pub fn handle_backslash(
    char_iter: &mut std::str::Chars,
) -> Result<Option<RegexStep>, &'static str> {
    let mut peekable_iter = char_iter.peekable();
    if let Some(&next) = peekable_iter.peek() {
        peekable_iter.next();
        // Avanzar el iterador después de procesar el backslash
        if peekable_iter.peek().is_some() {
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

pub fn check_min_max(min: Option<usize>, max: Option<usize>, counter: usize) -> bool {
    match min {
        Some(min) => match max {
            Some(max) => return !(counter < min || counter > max),
            None => {
                if counter > min {
                    return true;
                }
                false
            }
        },
        None => false,
    }
}

pub fn handle_brackets(char_iter: &mut std::str::Chars) -> Result<RegexStep, &'static str> {
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
                if char_iter.next() == Some(':') {
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

pub fn handle_curly(steps: &mut [RegexStep], char_iter: &mut std::str::Chars) -> Option<RegexStep> {
    let mut min = None;
    let mut max = None;
    let mut num_str = String::new();
    let mut after_comma = false;
    let mut no_comma = true;
    for ch in char_iter.by_ref() {
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
