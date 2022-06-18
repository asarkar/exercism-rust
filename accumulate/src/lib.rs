/// What should the type of _function be?
pub fn map<A, B, F>(input: Vec<A>, mut function: F) -> Vec<B>
where
    F: FnMut(A) -> B,
{
    let mut result = Vec::new();
    for item in input {
        result.push(function(item));
    }
    result
}
