use std::fmt::{Debug, Display};

pub const NUM_COLORS: usize = 2;
pub const COLORS: [u8; NUM_COLORS] = ['w' as u8, 'b' as u8];

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ActiveColor {
    color: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ActiveColorError {
    Invalid,
}

impl ActiveColor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(&self) -> char {
        self.color as char
    }
}

impl Default for ActiveColor {
    fn default() -> Self {
        Self { color: COLORS[0] }
    }
}

impl TryFrom<&str> for ActiveColor {
    type Error = ActiveColorError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.eq("w") {
            return Ok(Self { color: 'w' as u8 });
        }
        if value.eq("b") {
            return Ok(Self { color: 'b' as u8 });
        }
        Err(Self::Error::Invalid)
    }
}

impl TryFrom<String> for ActiveColor {
    type Error = ActiveColorError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

pub trait ActiveColorExt {
    fn switch(&mut self);
}

impl ActiveColorExt for ActiveColor {
    fn switch(&mut self) {
        self.color = if self.color == COLORS[0] { COLORS[1] } else { COLORS[0] }
    }
}

impl Display for ActiveColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color())
    }
}

impl Debug for ActiveColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_color_creation() {
        let active_color = ActiveColor::new();
        assert_eq!(active_color.color(), 'w');
    }

    #[test]
    fn test_default_active_color() {
        let default_active_color = ActiveColor::default();
        assert_eq!(default_active_color.color(), 'w');
    }

    #[test]
    fn test_active_color_switch() {
        let mut active_color = ActiveColor::new();
        active_color.switch();
        assert_eq!(active_color.color(), 'b');
        active_color.switch();
        assert_eq!(active_color.color(), 'w');
    }

    #[test]
    fn test_active_color_display() {
        let active_color = ActiveColor::new();
        assert_eq!(format!("{active_color}"), "w");
    }

    #[test]
    fn test_active_color_debug() {
        let active_color = ActiveColor::new();
        assert_eq!(format!("{active_color:?}"), "w");
    }

    #[test]
    fn test_try_from_valid_string() {
        let valid_str = "b";
        let result = ActiveColor::try_from(valid_str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().color(), 'b');
    }

    #[test]
    fn test_try_from_invalid_string() {
        let invalid_str = "invalid";
        let err = ActiveColor::try_from(invalid_str).unwrap_err();
        assert_eq!(err, ActiveColorError::Invalid);
    }

    #[test]
    fn test_try_from_valid_string_from_string() {
        let valid_str = String::from("b");
        let result = ActiveColor::try_from(valid_str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().color(), 'b');
    }

    #[test]
    fn test_try_from_invalid_string_from_string() {
        let invalid_str = String::from("invalid");
        let result = ActiveColor::try_from(invalid_str);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(ActiveColorError::Invalid));
    }
}
