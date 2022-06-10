use regex::Regex;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    type Binop = fn(i32, i32) -> i32;
    let my_pow: Binop = |x, y| i32::pow(x, y as u32);
    let phrases: HashMap<&str, Binop> = HashMap::from([
        ("plus", Add::add as Binop),
        ("minus", Sub::sub as Binop),
        ("multiplied by", Mul::mul as Binop),
        ("divided by", Div::div as Binop),
        ("raised to the", my_pow),
    ]);

    let mut prev_match: Option<regex::Match> = None;
    let mut result: Option<i32> = None;

    let number_re = Regex::new(r"[-]??\d+").unwrap();

    for mat in number_re.find_iter(command) {
        if let Some(prev_mat) = prev_match {
            let phrase = (&command[prev_mat.end()..mat.start()]).trim();
            if let Some(op) = phrases.get(phrase) {
                let num = mat.as_str().parse::<i32>().unwrap();
                result.replace(op(result.unwrap(), num));
                prev_match.replace(mat);
            } else {
                eprintln!("Unknown operation: {}", phrase);
                return None;
            }
        } else {
            prev_match.replace(mat);
            result.replace(mat.as_str().parse::<i32>().unwrap());
        }
    }

    // Handle error cases
    if let Some(prev_mat) = prev_match {
        // Command must end with a `number?` or `power?`
        if prev_mat.end() != command.len() - 1 && !command.ends_with("power?") {
            eprintln!("Invalid command: {}", command);
            return None;
        }
    } else {
        eprintln!("No numbers found: {}", command);
        return None;
    }

    result
}
