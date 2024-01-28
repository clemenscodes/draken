use api::{Square, SquareExt};
use std::fmt::{Debug, Display};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
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
    bits: u64,
}

impl Bitboard {
    fn new(bits: u64) -> Self {
        Bitboard { bits }
    }

    pub fn bits(&self) -> u64 {
        self.bits
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Bitboard { bits: u64::MIN }
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
        Bitboard::new(self.bits.wrapping_shl(rhs as u32))
    }
}

impl ShlAssign<usize> for Bitboard {
    fn shl_assign(&mut self, rhs: usize) {
        self.bits = self.bits.wrapping_shl(rhs as u32);
    }
}

impl Shr<usize> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self {
        Bitboard::new(self.bits.wrapping_shr(rhs as u32))
    }
}

impl ShrAssign<usize> for Bitboard {
    fn shr_assign(&mut self, rhs: usize) {
        self.bits = self.bits.wrapping_shr(rhs as u32);
    }
}

pub trait BitboardExt {
    fn get_single_bit(index: usize) -> Bitboard {
        SINGLE_BITS[index]
    }
    fn check_bit(bitboard: Bitboard, index: usize) -> bool {
        (bitboard & Bitboard::get_single_bit(index)).bits != 0
    }
    fn set_bit(bitboard: Bitboard, index: usize) -> Bitboard {
        bitboard | Bitboard::get_single_bit(index)
    }
    fn unset_bit(bitboard: Bitboard, index: usize) -> Bitboard {
        Bitboard::set_bit(bitboard, index) ^ Bitboard::get_single_bit(index)
    }
    fn merge_many(bitboards: Vec<Bitboard>) -> Bitboard {
        bitboards.iter().fold(Bitboard::default(), |acc, &bb| acc | bb)
    }
    fn overlap(lhs: Bitboard, rhs: Bitboard) -> bool {
        (lhs & rhs).bits != 0
    }
    fn shift(bitboard: Bitboard, steps: i8) -> Bitboard {
        let abs_steps = steps.abs() as u32;
        if steps < 0 {
            return bitboard >> abs_steps as usize;
        }
        bitboard << abs_steps as usize
    }
    fn self_check_bit(&self, index: usize) -> bool;
    fn self_set_bit(&mut self, bitboard: Bitboard, index: usize);
    fn self_unset_bit(&mut self, bitboard: Bitboard, index: usize);
    fn self_merge_many(&mut self, bitboards: Vec<Bitboard>);
    fn self_overlap(&self, rhs: Bitboard) -> bool;
    fn self_not(&mut self);
}

impl BitboardExt for Bitboard {
    fn self_check_bit(&self, index: usize) -> bool {
        Bitboard::check_bit(*self, index)
    }

    fn self_set_bit(&mut self, bitboard: Bitboard, index: usize) {
        self.bits = Bitboard::set_bit(bitboard, index).bits;
    }

    fn self_merge_many(&mut self, bitboards: Vec<Bitboard>) {
        self.bits = Bitboard::merge_many(bitboards).bits;
    }

    fn self_overlap(&self, rhs: Bitboard) -> bool {
        Bitboard::overlap(*self, rhs)
    }

    fn self_unset_bit(&mut self, bitboard: Bitboard, index: usize) {
        self.bits = Bitboard::unset_bit(bitboard, index).bits;
    }

    fn self_not(&mut self) {
        self.bits = !(self).bits;
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
        let bitboard = Bitboard::get_single_bit(2);
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00100000\n\
            ";
        assert_eq!(expected, bitboard.to_string());
    }

    #[test]
    fn test_bitboard_add() {
        let a = Bitboard::get_single_bit(0);
        let b = Bitboard::get_single_bit(1);
        let c = a + b;
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            11000000\n\
            ";
        assert_eq!(expected, c.to_string());
    }

    #[test]
    fn test_bitboard_add_assign() {
        let mut a = Bitboard::get_single_bit(0);
        let b = Bitboard::get_single_bit(1);
        a += b;
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            11000000\n\
            ";
        assert_eq!(expected, a.to_string());
    }

    #[test]
    fn test_bitboard_sub() {
        let a = Bitboard::get_single_bit(2);
        let b = Bitboard::get_single_bit(1);
        let c = a - b;
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            01000000\n\
            ";
        assert_eq!(expected, c.to_string());
    }

    #[test]
    fn test_bitboard_sub_assign() {
        let mut a = Bitboard::get_single_bit(2);
        let b = Bitboard::get_single_bit(1);
        a -= b;
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            01000000\n\
            ";
        assert_eq!(expected, a.to_string());
    }

    #[test]
    fn test_bitboard_bitand() {
        let a = Bitboard::get_single_bit(0);
        let b = Bitboard::get_single_bit(1);
        let c = a & b;
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
        assert_eq!(expected, c.to_string());
    }

    #[test]
    fn test_bitboard_bitand_assign() {
        let mut a = Bitboard::get_single_bit(0);
        let b = Bitboard::get_single_bit(1);
        a &= b;
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
        assert_eq!(expected, a.to_string());
    }

    #[test]
    fn test_bitboard_bitor() {
        let a = Bitboard::get_single_bit(0);
        let b = Bitboard::get_single_bit(1);
        let c = a | b;
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            11000000\n\
            ";
        assert_eq!(expected, c.to_string());
    }

    #[test]
    fn test_bitboard_bitor_assign() {
        let mut a = Bitboard::get_single_bit(0);
        let b = Bitboard::get_single_bit(1);
        a |= b;
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            11000000\n\
            ";
        assert_eq!(expected, a.to_string());
    }

    #[test]
    fn test_bitboard_bitxor() {
        let a = Bitboard::get_single_bit(0);
        let b = Bitboard::get_single_bit(1);
        let c = a ^ b;
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            11000000\n\
            ";
        assert_eq!(expected, c.to_string());
    }

    #[test]
    fn test_bitboard_bitxor_assign() {
        let mut a = Bitboard::get_single_bit(0);
        let b = Bitboard::get_single_bit(1);
        a ^= b;
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            11000000\n\
            ";
        assert_eq!(expected, a.to_string());
    }

    #[test]
    fn test_bitboard_not() {
        let a = Bitboard::get_single_bit(0);
        let b = !a;
        let expected = "\
            11111111\n\
            11111111\n\
            11111111\n\
            11111111\n\
            11111111\n\
            11111111\n\
            11111111\n\
            01111111\n\
            ";
        assert_eq!(expected, b.to_string());
    }

    #[test]
    fn test_self_not() {
        let mut a = Bitboard::get_single_bit(0);
        a.self_not();
        let expected = "\
            11111111\n\
            11111111\n\
            11111111\n\
            11111111\n\
            11111111\n\
            11111111\n\
            11111111\n\
            01111111\n\
            ";
        assert_eq!(expected, a.to_string());
    }

    #[test]
    fn test_bitboard_shl() {
        let a = Bitboard::get_single_bit(0);
        let b = a << 60;
        let expected = "\
            00001000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            ";
        assert_eq!(expected, b.to_string());
    }

    #[test]
    fn test_bitboard_shl_assign() {
        let mut a = Bitboard::get_single_bit(0);
        a <<= 60;
        let expected = "\
            00001000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            ";
        assert_eq!(expected, a.to_string());
    }

    #[test]
    fn test_bitboard_shr() {
        let a = Bitboard::get_single_bit(63);
        let b = a >> 60;
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00010000\n\
            ";
        assert_eq!(expected, b.to_string());
    }

    #[test]
    fn test_bitboard_shr_assign() {
        let mut a = Bitboard::get_single_bit(63);
        a >>= 60;
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00010000\n\
            ";
        assert_eq!(expected, a.to_string());
    }

    #[test]
    fn test_check_bit_true() {
        let bitboard = Bitboard::get_single_bit(2);
        let index = 2;
        assert!(Bitboard::check_bit(bitboard, index));
    }

    #[test]
    fn test_check_bit_false() {
        let bitboard = Bitboard::get_single_bit(2);
        let index = 3;
        assert!(!Bitboard::check_bit(bitboard, index));
    }

    #[test]
    fn test_self_check_bit_true() {
        let bitboard = Bitboard::get_single_bit(2);
        let index = 2;
        assert!(bitboard.self_check_bit(index));
    }

    #[test]
    fn test_self_check_bit_false() {
        let bitboard = Bitboard::get_single_bit(2);
        let index = 3;
        assert!(!bitboard.self_check_bit(index));
    }

    #[test]
    fn test_set_bit() {
        let bitboard = Bitboard::get_single_bit(2);
        let index = 3;
        let result = Bitboard::set_bit(bitboard, index);
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00110000\n\
            ";
        assert_eq!(expected, result.to_string());
    }

    #[test]
    fn test_merge_many() {
        let bitboards = vec![
            Bitboard::get_single_bit(2),
            Bitboard::get_single_bit(5),
            Bitboard::get_single_bit(7),
        ];
        let result = Bitboard::merge_many(bitboards);
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00100101\n\
            ";
        assert_eq!(expected, result.to_string());
    }

    #[test]
    fn test_self_set_bit() {
        let mut bitboard = Bitboard::default();
        let index = 3;
        bitboard.self_set_bit(Bitboard::get_single_bit(2), index);
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00110000\n\
            ";
        assert_eq!(expected, bitboard.to_string());
    }

    #[test]
    fn test_self_merge_many() {
        let mut bitboard = Bitboard::default();
        let bitboards = vec![
            Bitboard::get_single_bit(2),
            Bitboard::get_single_bit(5),
            Bitboard::get_single_bit(7),
        ];
        bitboard.self_merge_many(bitboards);
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00100101\n\
            ";
        assert_eq!(expected, bitboard.to_string());
    }

    #[test]
    fn test_overlap_true() {
        let lhs = Bitboard::get_single_bit(2);
        let rhs = Bitboard::get_single_bit(2);
        let result = Bitboard::overlap(lhs, rhs);
        assert!(result);
    }

    #[test]
    fn test_overlap_false() {
        let lhs = Bitboard::get_single_bit(2);
        let rhs = Bitboard::get_single_bit(3);
        let result = Bitboard::overlap(lhs, rhs);
        assert!(!result);
    }

    #[test]
    fn test_self_overlap_true() {
        let bitboard = Bitboard::get_single_bit(2);
        let result = bitboard.self_overlap(Bitboard::get_single_bit(2));
        assert!(result);
    }

    #[test]
    fn test_self_overlap_false() {
        let bitboard = Bitboard::get_single_bit(2);
        let result = bitboard.self_overlap(Bitboard::get_single_bit(3));
        assert!(!result);
    }

    #[test]
    fn test_unset_bit() {
        let bitboard = Bitboard::get_single_bit(2);
        let index = 2;
        let result = Bitboard::unset_bit(bitboard, index);
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
        assert_eq!(expected, result.to_string());
    }

    #[test]
    fn test_shift_left() {
        let bitboard = Bitboard::get_single_bit(2);
        let steps = 3;
        let result = Bitboard::shift(bitboard, steps);
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000100\n\
            ";
        assert_eq!(expected, result.to_string());
    }

    #[test]
    fn test_shift_right() {
        let bitboard = Bitboard::get_single_bit(4);
        let steps = -3;
        let result = Bitboard::shift(bitboard, steps);
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            01000000\n\
            ";
        assert_eq!(expected, result.to_string());
    }

    #[test]
    fn test_shift_wrap() {
        let bitboard = Bitboard::get_single_bit(0);
        let result = Bitboard::shift(bitboard, 64);
        assert_eq!(bitboard, result);
        let result = Bitboard::shift(bitboard, -64);
        assert_eq!(bitboard, result);
    }
}
