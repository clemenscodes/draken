use crate::square::Square;

pub trait GameExt {
    fn init_game(&mut self);
    fn start_game(&mut self);
    fn resign(&mut self);
    fn offer_draw(&self);
    fn accept_draw(&mut self);
    fn decline_draw(&self);
    fn claim_draw(&mut self);
    fn get_game_state(&self);
    fn get_moves(&self);
    fn promote_queen(&mut self);
    fn promote_rook(&mut self);
    fn promote_knight(&mut self);
    fn promote_bishop(&mut self);
    fn is_own_piece_on_square(&self, square: Square) -> bool;
    fn make_move(&mut self, source: Square, destination: Square) -> Result<(), ()>;
    fn ply(&self) -> u16;
}
