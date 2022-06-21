use anyhow::Error;
use fancy_regex::{self, Regex};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug, Default)]
pub struct Flags {
    line_numbers: bool,
    name_only: bool,
    case_insensitive: bool,
    invert: bool,
    whole_line: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut f: Flags = Default::default();

        f.line_numbers = flags.contains(&"-n");
        f.name_only = flags.contains(&"-l");
        f.case_insensitive = flags.contains(&"-i");
        f.invert = flags.contains(&"-v");
        f.whole_line = f.invert || flags.contains(&"-x");

        f
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let re = build_pattern(pattern, flags)?;
    let mut res = Vec::new();
    for f in files {
        let lines = find(f, &re, flags, files.len() > 1)?;
        res.extend(lines);
    }

    Ok(res)
}

fn build_pattern(pattern: &str, flags: &Flags) -> Result<Regex, fancy_regex::Error> {
    let mut pat = String::new();

    if flags.case_insensitive {
        // Case insensitive modifier
        // https://www.regular-expressions.info/modifiers.html
        pat.push_str(r"(?i)");
    }
    if flags.whole_line {
        // Start of line
        pat.push('^');
    }
    if flags.invert {
        // Negative lookahead
        // https://www.regular-expressions.info/lookaround.html
        pat.push_str(r"(?:(?!{}).)*");
    } else {
        pat.push_str(r"(?:{})");
    }
    if flags.whole_line {
        // End of line
        pat.push('$');
    }

    if let Some(idx) = pat.find("{}") {
        pat.replace_range(idx..idx + 2, pattern);
    } else {
        pat.push_str(pattern);
    }

    Regex::new(&pat)
}

fn read_lines<P>(file: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let f = File::open(file)?;
    Ok(BufReader::new(f).lines())
}

fn find(file: &str, re: &Regex, flags: &Flags, file_name: bool) -> Result<Vec<String>, Error> {
    let lines = read_lines(file)?;
    let mut res = Vec::new();

    for (i, line) in lines.enumerate() {
        let line = line?;
        let mat = re.is_match(&line)?;
        if mat {
            if flags.name_only {
                res.push(file.to_string());
                break;
            }
            let mut s = String::new();
            if file_name {
                s.push_str(&format!("{}:", file));
            }
            if flags.line_numbers {
                s.push_str(&format!("{}:", i + 1));
            }
            s.push_str(&line.to_string());
            res.push(s);
        }
    }

    Ok(res)
}
