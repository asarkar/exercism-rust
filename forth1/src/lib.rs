use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Vec<(Option<usize>, String)>>,
}

impl Forth {
    pub fn new() -> Forth {
        Default::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        println!("input={}", input);
        let tokens: Vec<String> = input.split_whitespace()
            .map(|x| x.trim().to_ascii_uppercase())
            .collect();

        self.parse(tokens, 0)
    }

    fn parse(&mut self, tokens: Vec<String>, i: usize) -> Result {
        if i == tokens.len() {
            return Ok(());
        }
        let word = tokens[i].as_str();
        if word == ":" {
            self.parse_defn(tokens, i)
        } else if let Some(defn) = self.words.get(word).cloned() {
            self.eval_defn(defn.last().unwrap().clone())
        } else {
            match word {
                "+" | "-" | "*" | "/" => self.eval_arithmetic(word),                
                "DUP" => self.dup(),
                "DROP" => self.drop(),
                "SWAP" => self.swap(),
                "OVER" => self.over(), 
                x => {
                    if let Ok(j) = x.parse::<i32>() {
                        println!("Push {} on stack={:?}", j, self.stack);
                        Ok(self.stack.push(j))
                    } else {
                        println!("Unknown word: {}", x);
                        Err(Error::UnknownWord)
                    }
                },
            }?;
            self.parse(tokens, i + 1)
        }
    }

    fn parse_defn(&mut self, tokens: Vec<String>, i: usize) -> Result {
        let word = &tokens[i + 1];

        let mut j = i + 2;
        while j < tokens.len() && tokens[j] != ";" {
            let k = if let Some(x) = self.words.get(&tokens[j]) {
                Some(x.len() - 1)
            } else {
                None
            };
            self.words.entry(word.clone()).or_default().push((k, tokens[j].clone()));
            j += 1;
        }

        println!("Definition: {} = {:?}", word, self.words[word]);
        self.parse(tokens, j + 1)
    }

    fn eval_defn(&mut self, defn: (Option<usize>, String)) -> Result {
        let (version, word) = defn;
        println!("Evaluate defn ({:?}, {:?})", version, word);
        if let Some(v) = version {
            self.eval_defn(self.words[&word][v].clone())
        } else {
            self.eval(&word)
        }
    }

    fn eval_arithmetic(&mut self, op: &str) -> Result {
        println!("Evaluate {} on stack={:?}", op, self.stack);
        if let Some((y, x)) = self.stack.pop().zip(self.stack.pop()) {
            match op {
                "+" => Ok(self.stack.push(x + y)),
                "-" => Ok(self.stack.push(x - y)),
                "*" => Ok(self.stack.push(x * y)),
                "/" if y > 0 => Ok(self.stack.push(x / y)),
                "/" => Err(Error::DivisionByZero),
                _ => panic!("Unknown operator: {}", op),
            }
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn dup(&mut self) -> Result {
        println!("Evaluate dup on stack={:?}", self.stack);
        if let Some(last) = self.stack.last().cloned() {
            Ok(self.stack.push(last))
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn drop(&mut self) -> Result {
        println!("Evaluate drop on stack={:?}", self.stack);
        if let Some(_) = self.stack.pop() {
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn swap(&mut self) -> Result {
        println!("Evaluate swap on stack={:?}", self.stack);
        if let Some((x, y)) = self.stack.pop().zip(self.stack.pop()) {
            self.stack.push(x);
            Ok(self.stack.push(y))
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn over(&mut self) -> Result {
        println!("Evaluate over on stack={:?}", self.stack);
        if let Some(x) = self.stack.get(self.stack.len() - 2).cloned() {
            Ok(self.stack.push(x))
        } else {
            Err(Error::StackUnderflow)
        }
    }
}
