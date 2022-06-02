use std::collections::HashSet;
use std::ops::ControlFlow;

pub fn check(candidate: &str) -> bool {
    candidate
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .try_fold(HashSet::new(), |mut acc, ch| {
            if ch.is_ascii_whitespace() || ch == '-' {
                ControlFlow::Continue(acc)
            } else if !acc.contains(&ch) {
                acc.insert(ch);
                ControlFlow::Continue(acc)
            } else {
                ControlFlow::Break(acc)
            }
        })
        .is_continue()
}
