use std::fmt::Display;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?

// How to store a function in a struct? https://stackoverflow.com/a/52934680/839733
// Generic struct with lifetime: https://stackoverflow.com/a/25921004/839733
// Generic lifetime: https://exercism.org/tracks/rust/exercises/fizzy/solutions/andregenereux
pub struct Matcher<T> {
    is_match: Box<dyn Fn(T) -> bool>,
    sub: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: Display,
    {
        Matcher {
            is_match: Box::new(matcher),
            sub: subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T>(Vec<Matcher<T>>);

impl<T: Display + Clone> Fizzy<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Fizzy(vec![])
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.0.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        // item may outlive self.0, take ownership of self.0
        iter.map(move |item| {
            let s: String = self
                .0
                .iter()
                .filter(|m| (m.is_match)(item.clone()))
                .map(|m| m.sub.clone())
                .collect();
            if s.is_empty() {
                item.to_string()
            } else {
                s
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: Display + Clone>() -> Fizzy<T> {
    Fizzy::<T>::new()
        .add_matcher(Matcher::new(|n| to_u8(n) % 3 == 0, "fizz"))
        .add_matcher(Matcher::new(|n| to_u8(n) % 5 == 0, "buzz"))
}

fn to_u8<T: Display>(n: T) -> u8 {
    n.to_string().parse::<u8>().unwrap()
}
