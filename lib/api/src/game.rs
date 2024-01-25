use crate::square::Square;

pub trait Game {
    fn init_game();
    fn start_game();
    fn resign();
    fn offer_draw();
    fn accept_draw();
    fn decline_draw();
    fn claim_draw();
    fn get_game_state();
    fn get_moves();
    fn make_move();
    fn promote_queen();
    fn promote_rook();
    fn promote_knight();
    fn promote_bishop();
    fn is_own_piece_on_scquare(square: Square) -> bool;
}
