use api::{Square, SquareExt};
use std::fmt::{Debug, Display};
use std::ops::{BitAnd, BitAndAssign};
use std::sync::{LazyLock, Mutex};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bitboard {
    pub bits: u64,
}

static SINGLE_BITS: LazyLock<Mutex<[Bitboard; u64::BITS as usize]>> = LazyLock::new(|| {
    let mut single_bits = [Bitboard::new_empty(); u64::BITS as usize];
    Square::iterate_square_indices(|rank, file| {
        let index: usize = Square::from_rank_file_to_index(rank, file);
        single_bits[index] = Bitboard::new(1u64 << index);
    });
    Mutex::new(single_bits)
});

pub trait BitboardExt {
    fn get_single_bit(index: usize) -> Bitboard {
        SINGLE_BITS
            .lock()
            .map(|boards| boards[index].to_owned())
            .unwrap()
    }
    fn check_bit(bitboard: Bitboard, index: usize) -> bool {
        (bitboard & Bitboard::get_single_bit(index)).bits != 0
    }
}

impl Bitboard {
    fn new(bits: u64) -> Self {
        Bitboard { bits }
    }
    fn new_empty() -> Self {
        Bitboard { bits: 0 }
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Bitboard::new(self.bits & rhs.bits)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits = (*self & rhs).bits;
    }
}

impl BitboardExt for Bitboard {}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Square::iterate_square_indices(|rank, file| {
            let index: usize = Square::from_rank_file_to_index(rank, file);
            let mask = Bitboard::get_single_bit(index);
            let intersection = if (*self & mask).bits != 0 { "1" } else { "0" };
            write!(f, "{intersection}").unwrap();
            if file == 7 {
                writeln!(f).unwrap();
            }
        });
        Ok(())
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[test]
fn test_get_single_bits() {
    let bit = Bitboard::get_single_bit(2);
    println!("{bit:#?}");
}
