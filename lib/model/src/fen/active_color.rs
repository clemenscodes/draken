use std::fmt::{Debug, Display};

pub const NUM_COLORS: usize = 2;
pub const COLORS: [char; NUM_COLORS] = ['w', 'b'];

pub struct ActiveColor {
    color: char,
}

impl ActiveColor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(&self) -> char {
        self.color
    }
}

impl Default for ActiveColor {
    fn default() -> Self {
        Self { color: COLORS[0] }
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
        write!(f, "{}", self.color)
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
}
