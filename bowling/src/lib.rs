#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
struct Frame(Vec<u16>);

impl Frame {
    pub fn new() -> Self {
        Default::default()
    }

    fn num_throws(&self) -> usize {
        self.0.len()
    }
    fn score(&self) -> u16 {
        self.0.iter().sum()
    }
    fn is_complete(&self) -> bool {
        self.num_throws() == 2 || self.score() == 10
    }
}

#[derive(Default)]
pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Default::default()
    }

    fn is_game_complete(&self) -> bool {
        let num_frames = self.frames.len();
        if num_frames < 10 {
            return false;
        }
        let tenth_frame_throws = self.frames[9].num_throws();
        let tenth_frame_score = self.frames[9].score();
        match num_frames {
            10 => tenth_frame_throws == 2 && tenth_frame_score < 10,
            11 => tenth_frame_throws + self.frames[10].num_throws() == 3,
            _ => true,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_complete() {
            eprintln!("Game is complete");
            return Err(Error::GameComplete);
        }
        let n = self.frames.len();
        let mut frame = if self.frames.is_empty() || self.frames[n - 1].is_complete() {
            Frame::new()
        } else {
            self.frames.pop().unwrap()
        };
        if frame.score() + pins > 10_u16 {
            eprintln!("Not enough pins left");
            return Err(Error::NotEnoughPinsLeft);
        }
        frame.0.push(pins);
        self.frames.push(frame);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_complete() {
            eprintln!("Game is not complete");
            return None;
        }

        let score = self.frames[..10]
            .iter()
            .enumerate()
            .fold(0_u16, |score, (i, f)| {
                let f_score = f.score();
                score
                    + f_score
                    + (match f_score {
                        10 if f.num_throws() == 1 => self.frames[i + 1..]
                            .iter()
                            .flat_map(|f| f.0.iter())
                            .take(2)
                            .sum(),
                        10 => self.frames[i + 1].0[0],
                        _ => 0,
                    })
            });
        Some(score)
    }
}
