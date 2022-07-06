mod cmd;
mod defn;

use cmd::Cmd;
use defn::Dict;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::any::{Any, TypeId};

#[derive(pest_derive::Parser)]
#[grammar = "forth.pest"]
pub struct ForthParser;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Default)]
pub struct Forth<'a> {
    cmd: Cmd,
    dict: Dict<'a>,
}

// ### pest references ###
// https://docs.rs/pest_derive/latest/pest_derive/
// https://pest.rs/book/intro.html
// Parser for Calculator: https://createlang.rs/01_calculator/calc_intro.html
// Parsing with Pest: https://www.youtube.com/watch?v=r935AKecsk4
// HTTP Request Parser using pest: https://protiumx.dev/blog/posts/an-http-request-parser-with-rust-and-pest.rs/

// ### Parser references ###
// Easy Forth: https://skilldrick.github.io/easyforth/
// Writing a Simple Parser in Rust: https://adriann.github.io/rust_parser.html

// ### General references ###
// Common Rust Lifetime Misconceptions: https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md

// The code is split into separate modules for better organization and abstraction.
// Module 'cmd' handles execution of commands, like `1 2 +`, and module 'defn'
// handles parsing and resolving word definitions, like `: foo 5 ;`.

// How the OOM attack works: x="some long string", "y = x x", "z = y y", etc. keeps appending the
// long string to every definition. If we resolve the definition eagerly, every definition is
// stored in this way, not only the final string. If we keep doing it, the string will become
// large enough to cause a crash.
// My solution prevents this attack by resolving the definition at runtime.
impl<'a> Forth<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn stack(&self) -> &[Value] {
        self.cmd.stack()
    }

    pub fn eval(&mut self, input: &'a str) -> Result {
        // println!("Process line: {}", input);
        match ForthParser::parse(Rule::Line, input) {
            Ok(line) => self.process_line(line),
            e => panic!("Failed to parse line: {}, {:?}", input, e),
        }
    }

    fn process_line(&mut self, pairs: Pairs<'a, Rule>) -> Result {
        pairs
            .flat_map(|p| p.into_inner())
            .try_fold((), |acc, p| match p.as_rule() {
                Rule::WordDefn => self.dict.new_word(p),
                Rule::Cmd => self.run_cmd(p),
                Rule::InvalidWord => {
                    eprintln!("Invalid word: {}", p.as_str());
                    Err(Error::InvalidWord)
                }
                _ => Ok(acc),
            })
    }

    fn run_cmd(&mut self, pair: Pair<'a, Rule>) -> Result {
        pair.into_inner()
            .peek()
            .map(|p| {
                let built_in = !self.dict.is_word(&p);
                match p.as_rule() {
                    Rule::BuiltInCmd => self.cmd.run_built_in_cmd(p),
                    Rule::Word => {
                        // Since built-in commands can be redefined,
                        // most tokens other than int come in as words.
                        if built_in {
                            self.run_built_in_cmd(&Dict::to_word(&p))
                        } else {
                            let defn = self.dict.resolve_defn(&p)?;
                            // println!("Word: {}, definition: {:?}", p.as_str(), defn);
                            defn.into_iter()
                                .try_fold((), |_, d| self.run_built_in_cmd(&d))
                        }
                    }
                    _ => panic!("Unknown command: {}", p),
                }
            })
            .unwrap()
    }

    fn is_parse_error(a: &dyn Any) -> bool {
        TypeId::of::<pest::error::Error<Rule>>() == a.type_id()
    }

    fn run_built_in_cmd(&mut self, word: &str) -> Result {
        ForthParser::parse(Rule::BuiltInCmd, word)
            .map_err(|e| {
                if Forth::is_parse_error(&e) {
                    Error::UnknownWord
                } else {
                    Error::StackUnderflow
                }
            })
            .and_then(|cmd| self.cmd.run_built_in_cmd(cmd.peek().unwrap()))
    }
}
