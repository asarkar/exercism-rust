// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

const EARTH_YEAR_SECONDS: f64 = 31557600.0;

// https://doc.rust-lang.org/stable/book/ch19-06-macros.html
// https://doc.rust-lang.org/rust-by-example/macros.html
// https://danielkeep.github.io/tlborm/book/mbe-README.html
// https://users.rust-lang.org/t/macro-to-handle-a-literal-string/64820
// https://stackoverflow.com/a/50223259/839733
// https://stackoverflow.com/a/24835982/839733
macro_rules! impl_planet {
    ($($t:ty;$years:literal),+) => {
        $(impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                (d.0 as f64) / (EARTH_YEAR_SECONDS * $years)
            }
        })*
    }
}

impl_planet!(
    Mercury;0.2408467, Venus;0.61519726, Earth;1.0, Mars;1.8808158,
    Jupiter;11.862615, Saturn;29.447498, Uranus;84.016846, Neptune;164.79132
);
