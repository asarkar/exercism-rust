use pest::iterators::Pair;
use pest::{self, Parser};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(pest_derive::Parser)]
#[grammar = "forth.pest"]
pub struct ForthParser;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Default)]
// https://stackoverflow.com/a/27590535/839733
pub struct Forth {
    stack: Vec<Value>,
    word_defn: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Default::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        println!("Evaluate: {}", input);
        let pairs = ForthParser::parse(Rule::Line, input);
        if pairs.is_err() {
            eprintln!("Stack underflow");
            return Err(Error::StackUnderflow);
        }
        let mut defn = false;

        for p in pairs.unwrap().flatten() {
            let res = match p.as_rule() {
                Rule::WordDefn => {
                    defn = true;
                    self.define_word(p)
                }
                Rule::Word if !defn => {
                    let word = p.as_str();
                    if let Some(d) = self.word_defn.get(&word.to_ascii_uppercase()).cloned() {
                        println!("in dict {:?}", p);
                        self.eval(&d)
                    } else if let Some(rule) = ForthParser::parse(Rule::StackOp, word)
                        .ok()
                        .and_then(|pairs| pairs.peek().and_then(|pair| pair.into_inner().next()))
                    {
                        println!("{:?}", rule);
                        self.stack_op(rule.as_rule())
                    } else if let Some(rule) = ForthParser::parse(Rule::BinOp, word)
                        .ok()
                        .and_then(|pairs| pairs.peek().and_then(|pair| pair.into_inner().next()))
                    {
                        println!("{:?}", rule);
                        self.binary_expr(rule.as_rule())
                    } else {
                        eprintln!("Unknown word: {}", word);
                        Err(Error::UnknownWord)
                    }
                }
                Rule::Int if !defn => {
                    println!("{:?}", p);
                    let i = p.as_str().parse::<Value>().unwrap();
                    self.stack.push(i);
                    Ok(())
                }
                Rule::BinOp if !defn => self.binary_expr(p.into_inner().next().unwrap().as_rule()),
                Rule::StackOp if !defn => self.stack_op(p.into_inner().next().unwrap().as_rule()),
                Rule::SEMICOLON => {
                    defn = false;
                    Ok(())
                }
                Rule::InvalidWord => Err(Error::InvalidWord),
                _ => Ok(()),
            };
            res?
        }
        Ok(())
    }

    fn define_word(&mut self, pair: Pair<Rule>) -> Result {
        let (words, defn): (Vec<(Rule, &str)>, Vec<(Rule, &str)>) = pair
            .into_inner()
            .flatten()
            .filter(|x| [Rule::Word, Rule::Defn].contains(&x.as_rule()))
            .map(|x| (x.as_rule(), x.as_str().trim()))
            .partition(|x| x.0 == Rule::Word);

        let mut buffer = VecDeque::new();
        if let Some((_, d)) = defn.first() {
            buffer.push_front(d.to_ascii_uppercase().trim().to_string());
        }

        for (_, word) in words.iter().skip(1).rev() {
            let w = word.to_ascii_uppercase().trim().to_string();
            if let Some(d) = self.word_defn.get(&w) {
                println!("Push {}", d);
                buffer.push_front(d.clone());
                if !buffer.is_empty() {
                    println!("Set {}={:?}", w, buffer);
                    self.word_defn.insert(w, self.join(&mut buffer));
                }
            } else if ForthParser::parse(Rule::Cmd, &w).is_ok() {
                println!("Push {}", w);
                buffer.push_front(w);
            } else if !buffer.is_empty() {
                println!("Set {}={:?}", w, buffer);
                self.word_defn.insert(w, self.join(&mut buffer));
            } else {
                eprintln!("Unknown word: {}", w);
                return Err(Error::UnknownWord);
            }
        }
        let w = words[0].1.to_ascii_uppercase().trim().to_string();

        self.word_defn.insert(w, self.join(&mut buffer));

        println!("word_defn={:?}", self.word_defn);

        Ok(())
    }

    fn join(&self, buffer: &mut VecDeque<String>) -> String {
        let s = buffer.iter().fold(String::new(), |mut s, x| {
            s.push_str(x);
            s.push(' ');
            s
        });
        s.trim().to_string()
    }

    fn binary_expr(&mut self, rule: Rule) -> Result {
        println!("Math on stack={:?}", self.stack);
        println!("{:?}", rule);
        if let Some((y, x)) = self.stack.pop().zip(self.stack.pop()) {
            let result = match rule {
                Rule::Add => x + y,
                Rule::Sub => x - y,
                Rule::Mul => x * y,
                Rule::Div if y != 0 => x / y,
                _ => return Err(Error::DivisionByZero),
            };
            self.stack.push(result);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn stack_op(&mut self, rule: Rule) -> Result {
        match rule {
            Rule::Dup => self.dup(),
            Rule::Drop => self.drop(),
            Rule::Swap => self.swap(),
            _ => self.over(),
        }
    }

    fn dup(&mut self) -> Result {
        println!("DUP on stack={:?}", self.stack);
        if let Some(last) = self.stack.last().cloned() {
            self.stack.push(last);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn drop(&mut self) -> Result {
        println!("DROP on stack={:?}", self.stack);
        if self.stack.pop().is_some() {
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn swap(&mut self) -> Result {
        println!("SWAP on stack={:?}", self.stack);
        if let Some((x, y)) = self.stack.pop().zip(self.stack.pop()) {
            self.stack.push(x);
            self.stack.push(y);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn over(&mut self) -> Result {
        println!("OVER on stack={:?}", self.stack);
        if self.stack.len() < 2 {
            Err(Error::StackUnderflow)
        } else {
            self.stack.push(self.stack[self.stack.len() - 2]);
            Ok(())
        }
    }
}
