#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq)]
enum FrameState {
    Ongoing,
    Open,
    Spare,
    Strike,
}

#[derive(Debug)]
struct Frame {
    pins: Vec<u16>,
    state: FrameState,
    score: u16,
}

impl Frame {
    fn new() -> Self {
        Self {
            pins: vec![],
            state: FrameState::Ongoing,
            score: 0,
        }
    }

    fn update(&mut self, pins: u16) -> Result<usize, Error> {
        self.pins.push(pins);
        self.score = self.pins.iter().sum();

        // Someone got too greedy
        if self.score > 10 {
            eprintln!("Not enough pins left");
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.score == 10 {
            match self.pins.len() {
                1 => self.state = FrameState::Strike,
                _ => self.state = FrameState::Spare,
            }
        }
        // The fill frame throw #1 is left in the Ongoing state
        // because in order to close it, we need to know the
        // status of the 10th frame, which we don't have here.
        else if self.score < 10 && self.pins.len() == 2 {
            self.state = FrameState::Open;
        }

        Ok(self.pins.len())
    }
}

#[derive(Default)]
pub struct BowlingGame(Vec<Frame>);

impl BowlingGame {
    pub fn new() -> Self {
        Default::default()
    }

    fn is_incomplete(&self) -> bool {
        match self.0.len() {
            // There is a 12th frame if frames 10, 11, and 12 are all strikes
            12 => false,
            // Strike in the 10th frame is awarded two fill balls, game is afoot
            11 if self.0[9].state == FrameState::Strike => self.0[10].pins.len() < 2,
            // Spare in the 10th frame is awarded one fill ball, game is up
            11 => false,
            // Spare in the 10th frame is awarded one fill ball, game is afoot
            10 => self.0[9].state != FrameState::Open,
            _ => true,
        }
    }

    // Get the nth latest frame, n=0 for the most recent
    fn nth_last_frame(&mut self, n: usize) -> Option<&mut Frame> {
        if self.0.len() < (1 + n) {
            return None;
        }
        let i = self.0.len() - 1 - n;
        self.0.get_mut(i)
    }

    // Previous frames that were a strike or a spare get bonus
    // points from the current throw
    fn update_previous_scores(&mut self, pins: u16, num_throw: usize) {
        if num_throw == 1 {
            // First throw in current frame, update last spare
            if let Some(frame) = self
                .nth_last_frame(1)
                .filter(|f| f.state == FrameState::Spare)
            {
                frame.score += pins;
            };
            // First throw in current frame, update last to last strike
            if let Some(frame) = self
                .nth_last_frame(2)
                .filter(|f| f.state == FrameState::Strike)
            {
                frame.score += pins;
            };
        }

        // Second throw in current frame, update last strike
        if let Some(frame) = self
            .nth_last_frame(1)
            .filter(|f| f.state == FrameState::Strike)
        {
            frame.score += pins;
        };
    }

    // Get the ongoing frame or create a new one
    fn get_or_create_frame(&mut self) -> &mut Frame {
        if self
            .nth_last_frame(0)
            .filter(|f| f.state == FrameState::Ongoing)
            .is_none()
        {
            self.0.push(Frame::new());
        }
        self.nth_last_frame(0).unwrap()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if !self.is_incomplete() {
            eprintln!("Game is complete");
            return Err(Error::GameComplete);
        }

        let frame = self.get_or_create_frame();
        let num_throw = frame.update(pins)?;

        self.update_previous_scores(pins, num_throw);

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_incomplete() {
            eprintln!("Game is not complete, cannot score");
            None
        } else {
            // Exclude the fill frames, their scores
            // have already been added to the 10th frame
            Some(self.0[..10].iter().map(|f| f.score).sum())
        }
    }
}
