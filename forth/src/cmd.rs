use super::Error;
use super::Result;
use super::Rule;
use super::Value;
use pest::iterators::Pair;

#[derive(Default, Debug)]
pub struct Cmd {
    stack: Vec<Value>,
}

impl Cmd {
    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn run_built_in_cmd(&mut self, pair: Pair<'_, Rule>) -> Result {
        // println!("Run built-in command: {}", pair.as_str());
        pair.into_inner()
            .peek()
            .map(|p| match p.as_rule() {
                Rule::Int => p
                    .as_str()
                    .trim()
                    .parse::<Value>()
                    .map(|i| self.stack.push(i))
                    .map_err(|e| panic!("{}", e)),
                Rule::BinOp => self.eval_bin_op(p),
                Rule::StackOp => self.eval_stack_op(p),
                _ => {
                    eprintln!("Unknown word: {}", p.as_str());
                    Err(Error::UnknownWord)
                }
            })
            .unwrap()
    }

    fn eval_bin_op(&mut self, pair: Pair<'_, Rule>) -> Result {
        // println!("Evaluate binary operation: {}", pair.as_str());
        pair.into_inner()
            .peek()
            .map(|p| {
                if let Some((y, x)) = self.stack.pop().zip(self.stack.pop()) {
                    let result = match p.as_rule() {
                        Rule::Add => x + y,
                        Rule::Sub => x - y,
                        Rule::Mul => x * y,
                        Rule::Div if y != 0 => x / y,
                        Rule::Div => {
                            eprintln!("Division by zero");
                            return Err(Error::DivisionByZero);
                        }
                        _ => panic!("Unknown rule in binary operation: {}", p),
                    };
                    self.stack.push(result);
                    Ok(())
                } else {
                    eprintln!("Stack underflow during binary operation");
                    Err(Error::StackUnderflow)
                }
            })
            .unwrap()
    }

    fn eval_stack_op(&mut self, pair: Pair<'_, Rule>) -> Result {
        // println!("Evaluate stack operation: {}", pair.as_str());
        pair.into_inner()
            .peek()
            .map(|p| match p.as_rule() {
                Rule::Dup => self.dup(),
                Rule::Drop => self.drop(),
                Rule::Swap => self.swap(),
                Rule::Over => self.over(),
                _ => panic!("Unknown rule in stack operation: {}", p),
            })
            .unwrap()
    }

    fn dup(&mut self) -> Result {
        if let Some(last) = self.stack.last().cloned() {
            self.stack.push(last);
            Ok(())
        } else {
            eprintln!("Stack underflow attempting 'dup'");
            Err(Error::StackUnderflow)
        }
    }

    fn drop(&mut self) -> Result {
        if self.stack.pop().is_some() {
            Ok(())
        } else {
            eprintln!("Stack underflow attempting 'drop'");
            Err(Error::StackUnderflow)
        }
    }

    fn swap(&mut self) -> Result {
        if let Some((x, y)) = self.stack.pop().zip(self.stack.pop()) {
            self.stack.push(x);
            self.stack.push(y);
            Ok(())
        } else {
            eprintln!("Stack underflow attempting 'swap'");
            Err(Error::StackUnderflow)
        }
    }

    fn over(&mut self) -> Result {
        if self.stack.len() < 2 {
            eprintln!("Stack underflow attempting 'over'");
            Err(Error::StackUnderflow)
        } else {
            self.stack.push(self.stack[self.stack.len() - 2]);
            Ok(())
        }
    }
}
