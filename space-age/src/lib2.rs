use planet_derive::Planet;
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

#[derive(Planet)]
#[years = 0.2408467]
pub struct Mercury;
#[derive(Planet)]
#[years = 0.61519726]
pub struct Venus;
#[derive(Planet)]
#[years = 1.0]
pub struct Earth;
#[derive(Planet)]
#[years = 1.8808158]
pub struct Mars;
#[derive(Planet)]
#[years = 11.862615]
pub struct Jupiter;
#[derive(Planet)]
#[years = 29.447498]
pub struct Saturn;
#[derive(Planet)]
#[years = 84.016846]
pub struct Uranus;
#[derive(Planet)]
#[years = 164.79132]
pub struct Neptune;
