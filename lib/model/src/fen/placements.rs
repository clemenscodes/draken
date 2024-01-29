use std::fmt::{Debug, Display};

use crate::pieces::PIECES;

pub const NUM_RANKS: usize = 8;
pub const NUM_FILES: usize = 8;

pub struct Placements {
    placements: [u64; NUM_RANKS],
}

impl Placements {
    pub fn new(placements: [u64; NUM_RANKS]) -> Self {
        Self { placements }
    }

    pub fn placements(&self) -> [u64; NUM_RANKS] {
        self.placements
    }
}

impl From<String> for Placements {
    fn from(value: String) -> Self {
        let mut placements = [0u64; NUM_RANKS];
        let split = value.split("/");
        if split.clone().count() != NUM_RANKS {
            panic!("Invalid piece placement data, too many ranks")
        }
        split.enumerate().for_each(|(num_rank, rank)| {
            println!("{num_rank}");
            let chars = rank.chars();
            if chars.clone().count() > NUM_FILES {
                panic!("Invalid piece placement data, too many files")
            }
            chars.for_each(|piece| {
                match piece {
                    p if p == PIECES[0] => {}
                    p if p == PIECES[1] => {}
                    p if p == PIECES[2] => {}
                    p if p == PIECES[3] => {}
                    p if p == PIECES[4] => {}
                    p if p == PIECES[5] => {}
                    p if p == PIECES[6] => {}
                    p if p == PIECES[0] => {}
                    p if p == PIECES[7] => {}
                    p if p == PIECES[8] => {}
                    p if p == PIECES[9] => {}
                    p if p == PIECES[10] => {}
                    p if p == PIECES[11] => {}
                    other => {
                        if !other.is_ascii_digit() {
                            panic!("Invalid piece placed")
                        }
                    }
                };
            });
        });
        Self { placements }
    }
}

impl Display for Placements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}

impl Debug for Placements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_placements() {
        let placements_data = [0u64; NUM_RANKS];
        let placements = Placements::new(placements_data);
        assert_eq!(placements.placements(), placements_data);
    }

    #[test]
    fn test_from_string() {
        let valid_placement_str = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        let placements = Placements::from(valid_placement_str);
        assert_eq!(placements.placements(), [0; NUM_RANKS]);
    }

    #[test]
    #[should_panic(expected = "Invalid piece placement data, too many ranks")]
    fn test_invalid_ranks() {
        std::panic::set_hook(Box::new(|_| {}));
        let invalid_placement_str = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/extra");
        let _ = Placements::from(invalid_placement_str);
    }

    #[test]
    #[should_panic(expected = "Invalid piece placement data, too many files")]
    fn test_invalid_files() {
        std::panic::set_hook(Box::new(|_| {}));
        let invalid_placement_str = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPPP/RNBQKBNR");
        let _ = Placements::from(invalid_placement_str);
    }

    #[test]
    #[should_panic(expected = "Invalid piece placed")]
    fn test_invalid_piece() {
        std::panic::set_hook(Box::new(|_| {}));
        let invalid_placement_str = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPJQ/RNBQKBNR");
        let _ = Placements::from(invalid_placement_str);
    }
}
