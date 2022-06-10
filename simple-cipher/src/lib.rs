use rand::distributions::Slice;
use rand::Rng;
use std::iter;

pub fn encode(key: &str, s: &str) -> Option<String> {
    xcode(key, s, |i, j| i + j)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    xcode(key, s, |i, j| i - j)
}

pub fn encode_random(s: &str) -> (String, String) {
    let ascii_lowercase: Vec<char> = (b'a'..b'z').map(|i| i as char).collect();
    let key: String = rand::thread_rng()
        .sample_iter(Slice::new(&ascii_lowercase[..]).unwrap())
        .take(101)
        .collect();

    let s = encode(&key, s).unwrap_or_default();
    (key, s)
}

fn is_invalid(s: &str) -> bool {
    s.is_empty() || s.chars().any(|c| !c.is_ascii_lowercase())
}

/*
 * Box<dyn Iterator..> indicates that we are returning a pointer to some
 * implementation of Iterator. Pretty cool!
 * https://dhghomon.github.io/easy_rust/Chapter_54.html
 *
 * Without explicit lifetime notation, 'key' has an anonymous lifetime but
 * the return type has static lifetime (the default). This doesn't compile,
 * because 'key' is used and required to live as long as the iterator.
 *
 * An anonymous lifetime '_ for the returned type doesn't work either,
 * because it can come from either 'key' or 's'. As explained above, 'key'
 * needs to live as long as the iterator.
 */
fn key_to_use<'a>(key: &'a str, s: &'a str) -> Box<dyn Iterator<Item = u8> + 'a> {
    if is_invalid(key) || is_invalid(s) {
        return Box::new(iter::empty::<u8>());
    }

    Box::new(key.bytes().chain(iter::repeat(b'a')).take(s.len()))
}

fn xcode<F: Fn(u8, u8) -> u8>(key: &str, s: &str, f: F) -> Option<String> {
    // No need to unbox explicitly
    let x = key_to_use(key, s)
        .zip(s.bytes())
        .map(|(k, ch)| {
            let offset = k - b'a';
            let mut c = f(ch, offset);
            let shift = b'z' - b'a' + 1;
            if c > b'z' {
                c -= shift;
            } else if c < b'a' {
                c += shift;
            };

            c as char
        })
        .collect::<String>();

    Some(x).filter(|x| !x.is_empty())
}
