pub const MAX_HALF_MOVE_CLOCK: usize = 150;
pub const MIN_HALF_MOVE_CLOCK: usize = 0;

#[derive(Debug)]
pub struct HalfMoveClock {
    clock: u8,
}
