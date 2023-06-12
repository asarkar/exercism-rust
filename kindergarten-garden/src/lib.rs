const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let pos = STUDENTS.binary_search(&student).unwrap() * 2;

    let to_plant = |c: char| match c {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => "",
    };
    diagram
        .lines()
        .flat_map(|line| line[pos..=pos + 1].chars().map(to_plant))
        .collect()
}
