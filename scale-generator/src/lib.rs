const SHARPS: &[&str] = &[
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];
const FLATS: &[&str] = &[
    "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
];
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub enum Error {
    InvalidTonic,
    InvalidInterval,
}
pub struct Scale {
    notes: Vec<String>,
}
// This question is probably the most unclear in the whole track masquerading
// as a music lesson.
// What is boils down to is given a starting point (tonic), and a variable number
// of steps (intervals), map each position to a character in one of the scales above.
// Treat the array as cyclic when the position is beyond it.
impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        // Find the scale to use
        let chromatic_scale = match tonic {
            "C" | "a" | "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#"
            | "d#" => SHARPS,
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "d" | "g" | "c" | "f" | "bb" | "eb" => FLATS,
            _ => return Err(Error::InvalidTonic),
        };
        // Find the note corresponding to the tonic
        let mut pos = chromatic_scale
            .iter()
            .position(|&n| n.to_uppercase() == tonic.to_uppercase())
            .unwrap();
        let mut notes = vec![chromatic_scale[pos].to_string()];
        println!(
            "tonic: {}, intervals: {}, pos: {}, notes: {:?}",
            tonic, intervals, pos, notes
        );
        // Translate each character in the interval to a step,
        // "half step" between two adjacent notes = 1,
        // "whole step" between two notes = 2,
        // "augmented second" is whole + half steps = 3
        for interval in intervals.chars() {
            pos += match interval {
                'm' => 1,
                'M' => 2,
                'A' => 3,
                _ => return Err(Error::InvalidInterval),
            };
            notes.push(chromatic_scale[pos % chromatic_scale.len()].to_string());
        }
        Ok(Self { notes })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        // I've no idea how this follows from the question
        Self::new(tonic, &"m".repeat(12))
    }
    pub fn enumerate(&self) -> Vec<String> {
        self.notes.clone()
    }
}
