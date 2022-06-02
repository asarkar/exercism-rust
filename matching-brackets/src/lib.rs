// LeetCode 20: Valid Parentheses.
pub fn brackets_are_balanced(string: &str) -> bool {
    let opening = vec!['(', '{', '['];
    let closing = vec![')', '}', ']'];
    let mut stack: Vec<char> = Vec::new();

    for ch in string.chars() {
        if let Some(pos) = closing.iter().position(|&x| x == ch) {
            if stack.pop() != opening.get(pos).copied() {
                return false;
            }
        } else if opening.iter().any(|&x| x == ch) {
            stack.push(ch);
        }
    }
    stack.is_empty()
}
