use std::ptr::eq;

use rgrep::structures::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
