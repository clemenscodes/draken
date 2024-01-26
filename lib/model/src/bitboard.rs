use api::{Square, SquareExt};
use std::fmt::{Debug, Display};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use std::sync::LazyLock;

static SINGLE_BITS: LazyLock<[Bitboard; u64::BITS as usize]> = LazyLock::new(|| {
    let mut single_bits = [Bitboard::default(); u64::BITS as usize];
    Square::iterate_square_indices(|rank, file| {
        let index: usize = Square::from_rank_file_to_index(rank, file);
        single_bits[index] = Bitboard::new(1u64 << index);
    });
    single_bits
});

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bitboard {
    pub bits: u64,
}

impl Bitboard {
    fn new(bits: u64) -> Self {
        Bitboard { bits }
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Bitboard { bits: 0 }
    }
}

impl Add for Bitboard {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Bitboard::new(self.bits + rhs.bits)
    }
}

impl AddAssign for Bitboard {
    fn add_assign(&mut self, rhs: Self) {
        self.bits += rhs.bits;
    }
}

impl Sub for Bitboard {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Bitboard::new(self.bits - rhs.bits)
    }
}

impl SubAssign for Bitboard {
    fn sub_assign(&mut self, rhs: Self) {
        self.bits -= rhs.bits;
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
        self.bits &= rhs.bits;
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Bitboard::new(self.bits | rhs.bits)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

impl BitXor for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Bitboard::new(self.bits ^ rhs.bits)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bits ^= rhs.bits;
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self {
        Bitboard::new(!self.bits)
    }
}

impl Shl<usize> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self {
        Bitboard::new(self.bits << rhs)
    }
}

impl ShlAssign<usize> for Bitboard {
    fn shl_assign(&mut self, rhs: usize) {
        self.bits <<= rhs;
    }
}

impl Shr<usize> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self {
        Bitboard::new(self.bits >> rhs)
    }
}

impl ShrAssign<usize> for Bitboard {
    fn shr_assign(&mut self, rhs: usize) {
        self.bits >>= rhs;
    }
}

pub trait BitboardExt {
    fn get_single_bit(index: usize) -> Bitboard {
        SINGLE_BITS[index]
    }
    fn check_bit(bitboard: Bitboard, index: usize) -> bool {
        (bitboard & Bitboard::get_single_bit(index)).bits != 0
    }
    fn self_check_bit(&self, index: usize) -> bool;
}

impl BitboardExt for Bitboard {
    fn self_check_bit(&self, index: usize) -> bool {
        Bitboard::check_bit(*self, index)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_bitboard_default() {
        let bitboard = Bitboard::default();
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            ";
        assert_eq!(expected, bitboard.to_string());
    }

    #[test]
    fn test_print_bitboard_max() {
        let bitboard = Bitboard::new(u64::MAX);
        let expected = "\
            11111111\n\
            11111111\n\
            11111111\n\
            11111111\n\
            11111111\n\
            11111111\n\
            11111111\n\
            11111111\n\
            ";
        assert_eq!(expected, bitboard.to_string());
    }

    #[test]
    fn test_get_single_bits() {
        let bit = Bitboard::get_single_bit(2);
        println!("{bit:#?}");
    }
}
