use std::collections::VecDeque;

enum RegexVal{
    Literal(char),
    Wildcard,

}

impl RegexVal{
    pub fn matches (&self, value:&str) -> usize {
        match self {
            RegexVal::Literal(l) => {
                if value.chars().next() == Some(*l){
                    l.len_utf8()
                } else{
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
        }
    }
}

struct RegexStep{
    val: RegexVal,
    rep: RegexRep,
}

enum RegexRep{
    Any,
    Exact(usize),
    Range{
        min: Option<usize>,
        max: Option<usize>,
    }
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
            return Err("El iput no es ascii");
        }

        let mut queue = VecDeque::from(self.steps);
        let mut index = 0;

        while let Some(step) = queue.pop_front(){
            match step.rep {
                RegexRep::Exact(n) => {
                    for _ in [1, n]{
                        let size =  step.val.matches(&value[index..]);
                        if size == 0{
                            return Ok(false);
                        }
                        index += size;
                    }
                }
                RegexRep::Any => {
                    let mut keep_matching = true;
                    while keep_matching{
                        let match_size = step.val.matches(&value[index..]);
                        if match_size != 0{
                            index += match_size;
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

