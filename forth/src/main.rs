use pest::{self, Parser};
use std::env;

#[derive(pest_derive::Parser)]
#[grammar = "forth.pest"]
pub struct ForthParser;

fn main() -> Result<(), pest::error::Error<Rule>> {
    let args: Vec<String> = env::args().collect();
    let pairs = ForthParser::parse(Rule::Line, &args[1])?.flatten();

    for p in pairs {
        println!("{:?}", p);
    }
    Ok(())
}
