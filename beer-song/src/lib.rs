pub fn verse(n: u32) -> String {
    let nb = num_bottles(n);
    let mut first_line = format!(
        "{} of beer on the wall, {} of beer.\n",
        nb,
        nb.to_lowercase()
    );
    let second_line = format!(
        "{}, {} of beer on the wall.\n",
        beginning_of_2nd_line(n),
        num_bottles_left(n)
    );

    first_line.push_str(&second_line);
    first_line
}

fn num_bottles(n: u32) -> String {
    if n == 0 {
        "No more bottles".to_string()
    } else if n == 1 {
        "1 bottle".to_string()
    } else {
        format!("{} bottles", n)
    }
}

fn num_bottles_left(n: u32) -> String {
    if n == 0 {
        "99 bottles".to_string()
    } else if n == 1 {
        "no more bottles".to_string()
    } else {
        num_bottles(n - 1)
    }
}

fn beginning_of_2nd_line(n: u32) -> String {
    if n == 0 {
        "Go to the store and buy some more".to_string()
    } else if n == 1 {
        "Take it down and pass it around".to_string()
    } else {
        "Take one down and pass it around".to_string()
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut lines = (end..=start).rev().fold(String::new(), |mut s, i| {
        s.push_str(&verse(i));
        s.push('\n');
        s
    });
    lines.pop();
    lines
}
