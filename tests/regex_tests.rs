use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use rgrep::class::Class;
use rgrep::evaluated_step::EvaluatedStep;
use rgrep::regex_rep::RegexRep;
use rgrep::regex::backtrack;
use rgrep::regex_step::{Regex, RegexStep};
use rgrep::regex_val::RegexVal;
use rgrep::utils::{check_min_max, handle_backslash, handle_brackets, handle_curly};
#[test]
fn test_regex_period() {
    let str_regex = "ab.cd";
    let filepath = "unit_test.txt";

    let mut regex_instance = match Regex::new(str_regex) {
        Ok(regex_instance) => regex_instance,
        Err(err) => {
            panic!("Error al crear la instancia de Regex: {}", err);
        }
    };

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => return,
    };
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => return,
        };
        lines.push(line);
    }

    for mut pattern in regex_instance {
        match pattern.test(&lines[0]) {
            Ok(result) => {
                assert_eq!(result, lines[0])
            }
            Err(err) => println!("Error applying the regular expression pattern: {}", err),
        }
    }
}

#[test]
fn test_regex_concatenance() {
    let str_regex = "ab.*cd";
    let filepath = "unit_test.txt";

    let mut regex_instance = match Regex::new(str_regex) {
        Ok(regex_instance) => regex_instance,
        Err(err) => {
            panic!("Error al crear la instancia de Regex: {}", err);
        }
    };

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => return,
    };
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => return,
        };
        lines.push(line);
    }

    for mut pattern in regex_instance {
        match pattern.test(&lines[1]) {
            Ok(result) => {
                assert_eq!(result, lines[1])
            }
            Err(err) => println!("Error applying the regular expression pattern: {}", err),
        }
    }
}

#[test]
fn test_regex_bracket() {
    let str_regex = "a[bc]d";
    let filepath = "unit_test.txt";

    let mut regex_instance = match Regex::new(str_regex) {
        Ok(regex_instance) => regex_instance,
        Err(err) => {
            panic!("Error al crear la instancia de Regex: {}", err);
        }
    };

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => return,
    };
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => return,
        };
        lines.push(line);
    }

    for mut pattern in regex_instance {
        match pattern.test(&lines[2]) {
            Ok(result) => {
                assert_eq!(result, lines[2])
            }
            Err(err) => println!("Error applying the regular expression pattern: {}", err),
        }
    }
}

#[test]
fn test_regex_simple_rep() {
    let str_regex = "ab{2,4}cd";
    let filepath = "unit_test.txt";

    let mut regex_instance = match Regex::new(str_regex) {
        Ok(regex_instance) => regex_instance,
        Err(err) => {
            panic!("Error al crear la instancia de Regex: {}", err);
        }
    };

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => return,
    };
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => return,
        };
        lines.push(line);
    }

    for mut pattern in regex_instance {
        match pattern.test(&lines[3]) {
            Ok(result) => {
                assert_eq!(result, lines[3])
            }
            Err(err) => println!("Error applying the regular expression pattern: {}", err),
        }
    }
}

#[test]
fn test_regex_alternance() {
    let str_regex = "abc|de+f";
    let filepath = "unit_test.txt";

    let mut regex_instance = match Regex::new(str_regex) {
        Ok(regex_instance) => regex_instance,
        Err(err) => {
            panic!("Error al crear la instancia de Regex: {}", err);
        }
    };

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => return,
    };
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => return,
        };
        lines.push(line);
    }

    let mut fullgrep = String::new();
    for mut pattern in regex_instance {
        match pattern.test(&lines[4]) {
            Ok(result) => {
                fullgrep.push_str(&result);
            }
            Err(err) => println!("Error applying the regular expression pattern: {}", err),
        }
    }
    assert_eq!(fullgrep, lines[4]);
}

#[test]
fn test_regex_bracket_2() {
    let str_regex = "la [aeiou] es una vocal";
    let filepath = "unit_test.txt";

    let mut regex_instance = match Regex::new(str_regex) {
        Ok(regex_instance) => regex_instance,
        Err(err) => {
            panic!("Error al crear la instancia de Regex: {}", err);
        }
    };

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => return,
    };
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => return,
        };
        lines.push(line);
    }

    let mut fullgrep = String::new();
    for mut pattern in regex_instance {
        match pattern.test(&lines[5]) {
            Ok(result) => {
                fullgrep.push_str(&result);
            }
            Err(err) => println!("Error applying the regular expression pattern: {}", err),
        }
    }
    assert_eq!(fullgrep, lines[5]);
}

#[test]
fn test_regex_negated_bracket() {
    let str_regex = "la [^aeiou] no es una vocal";
    let filepath = "unit_test.txt";

    let mut regex_instance = match Regex::new(str_regex) {
        Ok(regex_instance) => regex_instance,
        Err(err) => {
            panic!("Error al crear la instancia de Regex: {}", err);
        }
    };

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => return,
    };
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => return,
        };
        lines.push(line);
    }

    let mut fullgrep = String::new();
    for mut pattern in regex_instance {
        match pattern.test(&lines[6]) {
            Ok(result) => {
                fullgrep.push_str(&result);
            }
            Err(err) => println!("Error applying the regular expression pattern: {}", err),
        }
    }
    assert_eq!(fullgrep, lines[6]);
}

#[test]
fn test_regex_class_and_rep() {
    let str_regex = "hola [[:alpha:]]+";
    let filepath = "unit_test.txt";

    let mut regex_instance = match Regex::new(str_regex) {
        Ok(regex_instance) => regex_instance,
        Err(err) => {
            panic!("Error al crear la instancia de Regex: {}", err);
        }
    };

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => return,
    };
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => return,
        };
        lines.push(line);
    }

    let mut fullgrep = String::new();
    for mut pattern in regex_instance {
        match pattern.test(&lines[7]) {
            Ok(result) => {
                fullgrep.push_str(&result);
            }
            Err(err) => println!("Error applying the regular expression pattern: {}", err),
        }
    }
    assert_eq!(fullgrep, lines[7]);
}

#[test]
fn test_regex_class_2() {
    let str_regex = "[[:digit:]] es un numero";
    let filepath = "unit_test.txt";

    let mut regex_instance = match Regex::new(str_regex) {
        Ok(regex_instance) => regex_instance,
        Err(err) => {
            panic!("Error al crear la instancia de Regex: {}", err);
        }
    };

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => return,
    };
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => return,
        };
        lines.push(line);
    }

    let mut fullgrep = String::new();
    for mut pattern in regex_instance {
        match pattern.test(&lines[8]) {
            Ok(result) => {
                fullgrep.push_str(&result);
            }
            Err(err) => println!("Error applying the regular expression pattern: {}", err),
        }
    }
    assert_eq!(fullgrep, lines[8]);
}

#[test]
fn test_regex_class_3() {
    let str_regex = "el caracter [[:alnum:]] no es un simbolo";
    let filepath = "unit_test.txt";

    let mut regex_instance = match Regex::new(str_regex) {
        Ok(regex_instance) => regex_instance,
        Err(err) => {
            panic!("Error al crear la instancia de Regex: {}", err);
        }
    };

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => return,
    };
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => return,
        };
        lines.push(line);
    }

    let mut fullgrep = String::new();
    for mut pattern in regex_instance {
        match pattern.test(&lines[9]) {
            Ok(result) => {
                fullgrep.push_str(&result);
            }
            Err(err) => println!("Error applying the regular expression pattern: {}", err),
        }
    }
    assert_eq!(fullgrep, lines[9]);
}

#[test]
fn test_regex_anchor() {
    let str_regex = "es el fin$";
    let filepath = "unit_test.txt";

    let mut regex_instance = match Regex::new(str_regex) {
        Ok(regex_instance) => regex_instance,
        Err(err) => {
            panic!("Error al crear la instancia de Regex: {}", err);
        }
    };

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => return,
    };
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => return,
        };
        lines.push(line);
    }

    let mut fullgrep = String::new();
    for mut pattern in regex_instance {
        match pattern.test(&lines[10]) {
            Ok(result) => {
                fullgrep.push_str(&result);
            }
            Err(err) => println!("Error applying the regular expression pattern: {}", err),
        }
    }
    assert_eq!(fullgrep, lines[10]);
}


#[test]
fn test_backtrack() {
    let regex_step = RegexStep {
        rep: RegexRep::Exact(1),
        val: RegexVal::Literal('a'),
    };

    let mut evaluated_steps = Vec::new();
    evaluated_steps.push(EvaluatedStep {
        step: regex_step.clone(),
        size: 3, 
        backtrackeable: true,
    });

    let mut regex_queue = VecDeque::new();
    regex_queue.push_front(regex_step.clone());

    assert_eq!(backtrack(&regex_step, &mut evaluated_steps, &mut regex_queue), Some(3));
}

#[test]
fn test_matches_literal() {

    let regex_val = RegexVal::Literal('a');

    assert_eq!(regex_val.matches("abc"), 1);
    assert_eq!(regex_val.matches("xyz"), 0);
}

#[test]
fn test_matches_wildcard() {
    let regex_val = RegexVal::Wildcard;

    assert_eq!(regex_val.matches("abc"), 1);
    assert_eq!(regex_val.matches(""), 0); //No matchea con lineas vacias
}

#[test]
fn test_matches_bracket() {
    let regex_val = RegexVal::Bracket(vec!['a', 'e', 'i', 'o', 'u']);

    assert_eq!(regex_val.matches("ouoeuiieuauieo"), 1);
    assert_eq!(regex_val.matches("lkjhgfd"), 0);
}

#[test]
fn test_matches_negated_bracket() {
    let regex_val = RegexVal::NegatedBracket(vec!['a', 'e', 'i', 'o', 'u']);

    assert_eq!(regex_val.matches("ghytr"), 1);
    assert_eq!(regex_val.matches("iiee"), 0);
}

#[test]
fn test_matches_class() {
    let regex_val = RegexVal::Class(Class::Digit);

    assert_eq!(regex_val.matches("123"), 1);
    assert_eq!(regex_val.matches("abc"), 0);
}
