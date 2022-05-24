use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Include however many full hours in minutes, then
        // adjust. 25 hours means 1 o'clock.
        // 'h' may be negative, but h <= 23.
        let h = (hours + minutes / 60) % 24;
        // Having already included full hours, take remaining
        // minutes. 'm' may be negative, but m <= 59.
        let m = minutes % 60;

        // Get total minutes. Since h <= 23, and m <= 59,
        // 'total_minutes' is positive even if both 'h' and
        // 'm' are negative.
        let total_minutes = 24 * 60 + h * 60 + m;

        Clock {
            // Include however many full hours in total minutes,
            // then adjust.
            hours: (total_minutes / 60) % 24,
            // Having already included full hours, take remaining
            // minutes.
            minutes: total_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
