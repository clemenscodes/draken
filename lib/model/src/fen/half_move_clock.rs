use std::fmt::{Debug, Display};

pub const MAX_HALF_MOVE_CLOCK: u8 = 150;
pub const MIN_HALF_MOVE_CLOCK: u8 = 0;

pub struct HalfMoveClock {
    clock: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HalfMoveClockError {
    TooHigh,
    ParseError,
}

impl HalfMoveClock {
    pub fn new(clock: u8) -> Self {
        Self { clock }
    }

    pub fn clock(&self) -> u8 {
        self.clock
    }
}

impl Default for HalfMoveClock {
    fn default() -> Self {
        Self {
            clock: MIN_HALF_MOVE_CLOCK,
        }
    }
}

impl TryFrom<&str> for HalfMoveClock {
    type Error = HalfMoveClockError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(parsed_value) = value.parse::<u8>() {
            if parsed_value > MAX_HALF_MOVE_CLOCK {
                Err(Self::Error::TooHigh)
            } else {
                Ok(Self::new(parsed_value))
            }
        } else {
            Err(Self::Error::ParseError)
        }
    }
}

impl TryFrom<String> for HalfMoveClock {
    type Error = HalfMoveClockError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

pub(crate) trait HalfMoveClockExt {
    fn increment(&mut self);
}

impl HalfMoveClockExt for HalfMoveClock {
    fn increment(&mut self) {
        if self.clock < MAX_HALF_MOVE_CLOCK {
            self.clock += 1;
        }
    }
}

impl Display for HalfMoveClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clock)
    }
}

impl Debug for HalfMoveClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_str_valid() {
        let valid_value = "42";
        let result = HalfMoveClock::try_from(valid_value);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().clock(), 42);
    }

    #[test]
    fn test_try_from_str_too_high() {
        let too_high_value = "160";
        let err = HalfMoveClock::try_from(too_high_value).unwrap_err();
        assert_eq!(err, HalfMoveClockError::TooHigh);
    }

    #[test]
    fn test_try_from_str_too_low() {
        let too_low_value = "-5";
        let err = HalfMoveClock::try_from(too_low_value).unwrap_err();
        assert_eq!(err, HalfMoveClockError::ParseError);
    }

    #[test]
    fn test_try_from_str_parse_error() {
        let invalid_value = "abc";
        let err = HalfMoveClock::try_from(invalid_value).unwrap_err();
        assert_eq!(err, HalfMoveClockError::ParseError);
    }

    #[test]
    fn test_try_from_string_valid() {
        let valid_value = String::from("75");
        let res = HalfMoveClock::try_from(valid_value).unwrap();
        assert_eq!(res.clock(), 75);
    }

    #[test]
    fn test_increment_below_max() {
        let mut clock = HalfMoveClock::new(5);
        clock.increment();
        assert_eq!(clock.clock(), 6);
    }

    #[test]
    fn test_increment_at_max() {
        let mut clock = HalfMoveClock::new(MAX_HALF_MOVE_CLOCK);
        clock.increment();
        assert_eq!(clock.clock(), MAX_HALF_MOVE_CLOCK);
    }

    #[test]
    fn test_display() {
        let clock = HalfMoveClock::new(10);
        let formatted = format!("{}", clock);
        assert_eq!(formatted, "10");
    }

    #[test]
    fn test_debug() {
        let clock = HalfMoveClock::new(15);
        let debug_output = format!("{:?}", clock);
        assert_eq!(debug_output, "15");
    }
}
