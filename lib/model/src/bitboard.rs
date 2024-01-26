use api::{Square, SquareExt};
use std::fmt::Display;
use std::sync::{LazyLock, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bitboard {
    pub bits: u64,
}

const NUM_BITS: usize = 64;

static SINGLE_BITS: LazyLock<Mutex<[Bitboard; NUM_BITS]>> =
    LazyLock::new(|| {
        let mut single_bits = [Bitboard { bits: 0 }; NUM_BITS];
        for i in 0..NUM_BITS {
            let bitboard = Bitboard::new(1u64 << i);
            single_bits[i] = bitboard;
        }
        Mutex::new(single_bits)
    });

pub trait BitboardExt {
    fn get_single_bit(index: usize) -> Bitboard {
        SINGLE_BITS
        .lock()
        .map(|boards| boards[index].to_owned())
        .unwrap_or_else(|err| {
            // Handle the error (e.g., print an error message)
            eprintln!("Error locking SINGLE_BITS: {}", err);
            // Return a default Bitboard or handle the error accordingly
            Bitboard { bits: 0 }
        })
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

impl BitboardExt for Bitboard {}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in 7..=0 {
            for file in 0..=8 {
                let index: u8 = Square::from_rank_file(rank, file).into();

            }
        }
        Ok(())
    }
}

#[test]
fn test_get_single_bits() {
    let bit = Bitboard::get_single_bit(2);
    println!("{bit}");
}
