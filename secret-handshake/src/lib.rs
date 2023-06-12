pub fn actions(num: u8) -> Vec<&'static str> {
    let actions = vec!["wink", "double blink", "close your eyes", "jump"];

    let mut hs: Vec<&str> = actions
        .into_iter()
        .enumerate()
        .filter_map(|(n, a)| Some(a).filter(|_| ((1 << n) & num) > 0))
        .collect();

    if ((1 << 4) & num) > 0 {
        hs.reverse();
    }
    hs
}
