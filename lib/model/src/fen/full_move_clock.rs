use std::fmt::{Debug, Display};

pub const MIN_FULL_MOVE_CLOCK: u16 = 1;

pub struct FullMoveClock {
    clock: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FullMoveClockError {
    TooLow,
    ParseError,
}

impl FullMoveClock {
    pub fn new(clock: u16) -> Self {
        Self { clock }
    }

    pub fn clock(&self) -> u16 {
        self.clock
    }
}

impl Default for FullMoveClock {
    fn default() -> Self {
        Self {
            clock: MIN_FULL_MOVE_CLOCK,
        }
    }
}

impl TryFrom<&str> for FullMoveClock {
    type Error = FullMoveClockError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(parsed_value) = value.parse::<u16>() {
            if parsed_value < MIN_FULL_MOVE_CLOCK {
                Err(Self::Error::TooLow)
            } else {
                Ok(Self::new(parsed_value))
            }
        } else {
            Err(Self::Error::ParseError)
        }
    }
}

impl TryFrom<String> for FullMoveClock {
    type Error = FullMoveClockError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

pub(crate) trait FullMoveClockExt {
    fn increment(&mut self);
}

impl FullMoveClockExt for FullMoveClock {
    fn increment(&mut self) {
        self.clock += 1;
    }
}

impl Display for FullMoveClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clock)
    }
}

impl Debug for FullMoveClock {
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
        let result = FullMoveClock::try_from(valid_value);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().clock(), 42);
    }

    #[test]
    fn test_try_from_str_too_low() {
        let too_low_value = "0";
        let result = FullMoveClock::try_from(too_low_value);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(FullMoveClockError::TooLow));
    }

    #[test]
    fn test_try_from_str_parse_error() {
        let invalid_value = "abc";
        let result = FullMoveClock::try_from(invalid_value);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(FullMoveClockError::ParseError));
    }

    #[test]
    fn test_try_from_string_valid() {
        let valid_value = String::from("75");
        let result = FullMoveClock::try_from(valid_value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().clock(), 75);
    }

    #[test]
    fn test_try_from_string_invalid() {
        let invalid_value = String::from("invalid");
        let result = FullMoveClock::try_from(invalid_value);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(FullMoveClockError::ParseError));
    }

    #[test]
    fn test_increment() {
        let mut clock = FullMoveClock::new(5);
        clock.increment();
        assert_eq!(clock.clock(), 6);
    }

    #[test]
    fn test_display() {
        let clock = FullMoveClock::new(10);
        let formatted = format!("{}", clock);
        assert_eq!(formatted, "10");
    }

    #[test]
    fn test_debug() {
        let clock = FullMoveClock::new(15);
        let debug_output = format!("{:?}", clock);
        assert_eq!(debug_output, "15");
    }
}
