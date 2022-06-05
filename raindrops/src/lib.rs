pub fn raindrops(n: u32) -> String {
    let result: String = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .into_iter()
        .filter(|(i, _)| n % i == 0)
        .map(|(_, w)| w)
        .collect();

    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
