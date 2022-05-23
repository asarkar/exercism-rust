use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.is_empty() {
        return None;
    }

    let mut stack: Vec<i32> = Vec::new();
    type Binop = fn(i32, i32) -> i32;

    for item in inputs {
        let op: Option<Binop> = match item {
            CalculatorInput::Add => Some(Add::add),
            CalculatorInput::Subtract => Some(Sub::sub),
            CalculatorInput::Multiply => Some(Mul::mul),
            CalculatorInput::Divide => Some(Div::div),
            CalculatorInput::Value(i) => {
                stack.push(*i);
                None
            }
        };
        if let Some(x) = op {
            match stack.pop().zip(stack.pop()).map(|(i, j)| x(j, i)) {
                // Operands are reversed
                Some(y) => stack.push(y),
                // Not enough numbers
                _ => return None,
            }
        };
    }
    // Not enough operators
    if stack.len() > 1 {
        return None;
    }
    stack.pop()
}
