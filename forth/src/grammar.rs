// Nested enum: https://stackoverflow.com/a/55032729/839733
// Grouping structs with enums: https://stackoverflow.com/q/29088633/839733
// Optimizing layout of nested enums? https://internals.rust-lang.org/t/optimizing-layout-of-nested-enums/5098
// Error handliing: https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html
// Error Handling In Rust: https://www.lpalmieri.com/posts/error-handling-rust/
// Beginner's guide to Error Handling in Rust: https://www.sheshbabu.com/posts/rust-error-handling/
// Nicer error reporting: https://rust-cli.github.io/book/tutorial/errors.html
// Error Handling: https://rust-lang-nursery.github.io/rust-cookbook/errors/handle.html
// Pattern matching with multiple types: https://stackoverflow.com/a/50279877/839733
use pest::Parser;
use pest::iterators::{Pairs, Pair};
use anyhow::bail;

#[derive(pest_derive::Parser)]
#[grammar = "forth.pest"]
pub struct ForthParser;

#[derive(Debug)]
pub enum BinOp {
	Add,
	Sub,
	Mul,
	Div,
}

#[derive(Debug)]
pub enum StackOp {
	Dup,
	Drop,
	Swap,
	Over,
}

#[derive(Debug)]
pub enum BuiltInCmd {
	Int(i32),
	CompoundExpr(String),
	BinOp(BinOp),
	StackOp(StackOp),
}

#[derive(Debug)]
pub struct Word(pub String);

impl Word {
	pub fn new(s: &str) -> Self {
		Self(s.trim().to_ascii_uppercase())
	}
}

#[derive(Debug)]
pub enum Cmd {
	Word(Word),
	BuiltInCmd(BuiltInCmd, String),
}

#[derive(Debug)]
pub struct WordDefn {
	pub word: Word,
	pub defn: Vec<Cmd>,
}

#[derive(Debug)]
pub enum LineItem {
	WordDefn(WordDefn),
	Cmd(Cmd),
	InvalidWord(String),
}

pub type PestError = pest::error::Error<Rule>;

#[derive(Debug)]
pub struct Line(pub Vec<LineItem>);

impl<'i> TryFrom<Pairs<'i, Rule>> for Line {
	type Error = anyhow::Error;

	fn try_from(pairs: Pairs<'i, Rule>) -> Result<Self, Self::Error> {
		let items = pairs
			.flat_map(|p| p.into_inner())
			.map(|p| {
				match p.as_rule() {
					Rule::WordDefn => p.try_into().map(|x| LineItem::WordDefn(x)),
					Rule::Cmd => p.try_into().map(|x| LineItem::Cmd(x)),
					Rule::InvalidWord => Ok(LineItem::InvalidWord(p.as_str().trim().to_string())),
					_ => bail!("Unknown Cmd: {}", p.as_str()),
				}
			})
			.collect::<Result<Vec<LineItem>, Self::Error>>()?;

		Ok(Line(items))
	}
}

impl<'i> TryFrom<Pair<'i, Rule>> for WordDefn {
    type Error = anyhow::Error;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
    	let mut word = None;
    	let mut defn = vec![];

    	for p in pair.into_inner() {
    		match p.as_rule() {
        		Rule::Word => word = Some(Word::new(p.as_str())),
        		Rule::Defn => {
        			defn = p.into_inner()
						.map(|p| p.try_into())
						.collect::<Result<Vec<Cmd>, Self::Error>>()
						.unwrap();
				},
				_ => {},
        	};
    	}

       Ok(Self { word: word.unwrap(), defn })
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Cmd {
	type Error = anyhow::Error;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
    	pair.into_inner()
    		.peek()
    		.map(|p| {
    			let x = match p.as_rule() {
		    		Rule::Word => Cmd::Word(Word::new(p.as_str())),
		    		Rule::BuiltInCmd => {
		    			let s = p.as_str().to_string();
		    			Cmd::BuiltInCmd(p.try_into()?, s)
		    		},
		    		_ => bail!("Unknown Cmd: {}", p.as_str()),
		    	};
		    	Ok(x)
    		})
    		.unwrap()
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for BuiltInCmd {
	type Error = anyhow::Error;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
    	pair.into_inner()
    		.peek()
    		.map(|p| {
    			let x = match p.as_rule() {
		    		Rule::Int => BuiltInCmd::Int(p.as_str().trim().parse::<i32>()?),
					Rule::CompoundExpr => BuiltInCmd::CompoundExpr(p.as_str().trim().to_string()),
					Rule::BinOp => BuiltInCmd::BinOp(p.try_into()?),
					Rule::StackOp => BuiltInCmd::StackOp(p.try_into()?),
					_ => bail!("Unknown BuiltInCmd: {}", p.as_str()),
	    		};
	    		Ok(x)
    		})
    		.unwrap()
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for StackOp {
	type Error = anyhow::Error;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
    	pair.into_inner()
    		.peek()
    		.map(|p| {
    			let x = match p.as_rule() {
		    		Rule::Dup => StackOp::Dup,
		    		Rule::Drop => StackOp::Drop,
		    		Rule::Swap => StackOp::Swap,
		    		Rule::Over => StackOp::Over,
		    		_ => bail!("Unknown StackOp: {}", p.as_str()),
			    	};
	    		Ok(x)
    		})
    		.unwrap()
   	}
}

impl<'i> TryFrom<Pair<'i, Rule>> for BinOp {
	type Error = anyhow::Error;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
    	pair.into_inner()
    		.peek()
    		.map(|p| {
    			let x = match p.as_rule() {
		    		Rule::Add => BinOp::Add,
		    		Rule::Sub => BinOp::Sub,
		    		Rule::Mul => BinOp::Mul,
		    		Rule::Div => BinOp::Div,
		    		_ => bail!("Unknown BinOp: {}", p.as_str()),
		    	};
    			Ok(x)
    		})
    		.unwrap()
   	}
}

pub fn try_parse_line(s: &str) -> Result<Line, anyhow::Error> {
	ForthParser::parse(Rule::Line, s)?
		.try_into()
}

pub fn try_parse_built_in_cmd(s: &str) -> Result<BuiltInCmd, anyhow::Error> {
	ForthParser::parse(Rule::BuiltInCmd, s)?
		.peek()
		.map(|p| p.try_into())
		.unwrap_or_else(|| panic!("Couldn't parse {} into BuiltInCmd", s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
    	let input = &["1 2 3 4 5", "1 2 3 4 DROP Drop drop", "1 2 +", ": countup 1 2 3 ;", ": one 1 ; : two 2 ; one two +"];
    	for s in input {
    		let line: Line = try_parse_line(s).unwrap();
    		println!("'{}' => {:?}", s, line);
    	}
	}

	#[test]
    fn test_parse_built_in_cmd() {
    	let input = &["1"];
    	for s in input {
    		let cmd: BuiltInCmd = try_parse_line(s).unwrap();
    		println!("'{}' => {:?}", s, cmd);
    	}
    }
}
