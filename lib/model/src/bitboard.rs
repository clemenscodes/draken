use api::{Square, SquareExt};
use std::fmt::{Debug, Display};
use std::ops::BitAnd;
use std::sync::{LazyLock, Mutex};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bitboard {
    pub bits: u64,
}

const NUM_BITS: usize = 64;

static SINGLE_BITS: LazyLock<Mutex<[Bitboard; NUM_BITS]>> = LazyLock::new(|| {
    let mut single_bits = [Bitboard::new_empty(); NUM_BITS];
    Square::iterate_square_indices(|_rank, _file, index| {
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
}

impl Bitboard {
    fn new_empty() -> Self {
        Bitboard { bits: 0 }
    }
    fn new(bits: u64) -> Self {
        Bitboard { bits }
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard::new(self.bits & rhs.bits)
    }
}

impl BitboardExt for Bitboard {}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Square::iterate_square_indices(|_rank, file, index| {
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
