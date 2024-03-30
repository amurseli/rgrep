use std::collections::VecDeque;

struct EvaluatedStep{
    step:RegexStep,
    size: usize,
    backtrackeable: bool
    
}

#[derive(Debug)]
enum RegexVal{
    Literal(char),
    Wildcard,
    Bracket(Vec<char>),
    NegatedBracket(Vec<char>),

}

impl RegexVal{
    pub fn matches (&self, value:&str) -> usize {
        match self {
            RegexVal::Literal(l) => {
                if value.chars().next() == Some(*l){
                    println!("El caracter {:?} coincidio",l);
                    l.len_utf8()
                } else{
                    println!("El caracter {:?} NO coincidio con {:?}",l, value.chars().next());
                    0
                }
            },
            RegexVal::Wildcard => {
                if let Some(w) = value.chars().next(){
                    w.len_utf8()
                } else{
                    0
                }
            },
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
            },
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
        }
    }
}

#[derive(Debug)]
struct RegexStep{
    val: RegexVal,
    rep: RegexRep,
}

#[derive(Debug)]
enum RegexRep{
    Any,
    Exact(usize),
    Range{
        min: Option<usize>,
        max: Option<usize>,
    },
    //Bracket(Vec<char>),       
}

pub struct Regex{
    steps: Vec<RegexStep>
}

impl Regex{
    pub fn new(expression: &str) -> Result<Self, &str> {

        let mut steps: Vec<RegexStep> = Vec::new();
        
        let mut char_iter = expression.chars();
        while let Some(c) = char_iter.next(){
            let step = match c {
                '.' => Some(RegexStep{ 
                    rep: RegexRep::Exact(1), 
                    val: RegexVal::Wildcard 
                }),
                'a'..='z' => Some(RegexStep{
                    rep:RegexRep::Exact(1),
                    val:RegexVal::Literal(c)
                }),
                '*' => {
                    if let Some(last) = steps.last_mut(){
                        last.rep = RegexRep::Any;
                    }else{
                        return Err("'*' Inesperado")
                    }
                    None
                }
                '[' => {
                    let mut chars = Vec::new();
                    let mut negate = false;
                    let mut closed = false;
                    while let Some(ch) = char_iter.next() {
                        match ch {
                            ']' => {
                                closed = true;
                                break;
                            }
                            '^' => negate = true,
                            _ => chars.push(ch),
                        }
                    }
                    if !closed {
                        return Err("No closing bracket found");
                    }
                    let val = if negate {
                        RegexVal::NegatedBracket(chars)
                    } else {
                        RegexVal::Bracket(chars)
                    };
                    Some(RegexStep {
                        rep: RegexRep::Exact(1),
                        val,
                    })
                },
                ' ' => Some(RegexStep{
                    rep: RegexRep::Exact(1),
                    val: RegexVal::Literal(c)
                }),

                _ => return Err("Caracter Inesperado")

            };
            if let Some(p) = step{
                steps.push(p);

            }
        }

        
        
        Ok(Regex{steps})
        
    }

    pub fn test(self, value: &str) -> Result<bool, &str>{
        if !value.is_ascii(){
            return Err("El input no es ascii");
        }

        let mut queue = VecDeque::from(self.steps);
        let mut stack: Vec<EvaluatedStep> = Vec::new();
        let mut index = 0;
        'steps: while let Some(step) = queue.pop_front(){
            match step.rep {
                RegexRep::Exact(n) => {
                    let mut match_size = 0;
                    for _ in 0..=0{ //mirar este for
                        println!("n {:?}", step.val);
                        let size =  step.val.matches(&value[index..]);
                        if size == 0{
                            match backtrack(step, &mut stack, &mut queue){
                                Some(size) =>{
                                    index -= size;
                                    continue 'steps;
                                }
                                None => return Ok(false),
                            }
                        } else{
                            match_size += size;
                            index += size;
                            println!("Index {:?}", index)
                        }
                    }
                    stack.push(EvaluatedStep{
                        step: step,
                        size: match_size,
                        backtrackeable: false

                    })
                }
                RegexRep::Any => {
                    let mut keep_matching = true;
                    println!("n {:?}", step.val);
                    while keep_matching{        
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
                        }else{
                            keep_matching = false;
                        }
                    }
                },
                RegexRep::Range{min, max} => todo!()
            }
        }

        Ok(true)
    }
}

fn backtrack(
    current: RegexStep,
    evaluated: &mut Vec<EvaluatedStep>,
    next: &mut VecDeque<RegexStep>,
 ) -> Option<usize> {
    let mut back_size = 0;
    next.push_front(current);
    while let Some(e) = evaluated.pop(){
    back_size += e.size;
    if e.backtrackeable{
        println!("Backtrack {:?}", back_size);
        return Some(back_size);
    } else {
        next.push_front(e.step);
    }
    }
    
    None
}

