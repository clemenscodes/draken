use api::{Controller, ForsythEdwardsNotationExt, Game, Square};

#[derive(Debug, Clone)]
pub struct ChessController {}

impl ChessController {
    pub fn new() -> Self {
        ChessController {}
    }
}

impl Controller for ChessController {
    fn handle_mouse_pressed() {
        todo!()
    }

    fn handle_mouse_dragged() {
        todo!()
    }

    fn handle_mouse_released() {
        todo!()
    }

    fn handle_mouse_moved() {
        todo!()
    }

    fn get_error_message() -> String {
        todo!()
    }

    fn clear_error_message() {
        todo!()
    }

    fn get_fen() -> String {
        todo!()
    }

    fn get_source() -> api::Square {
        todo!()
    }

    fn get_destination() -> api::Square {
        todo!()
    }

    fn get_dragged_square() -> api::Square {
        todo!()
    }

    fn get_legal_moves() -> Vec<(Square, Square)> {
        todo!()
    }
}

impl Game for ChessController {
    fn init_game() {
        todo!()
    }

    fn start_game() {
        todo!()
    }

    fn resign() {
        todo!()
    }

    fn offer_draw() {
        todo!()
    }

    fn accept_draw() {
        todo!()
    }

    fn decline_draw() {
        todo!()
    }

    fn claim_draw() {
        todo!()
    }

    fn get_game_state() {
        todo!()
    }

    fn get_moves() {
        todo!()
    }

    fn make_move() {
        todo!()
    }

    fn promote_queen() {
        todo!()
    }

    fn promote_rook() {
        todo!()
    }

    fn promote_knight() {
        todo!()
    }

    fn promote_bishop() {
        todo!()
    }

    fn is_own_piece_on_square(_square: api::Square) -> bool {
        todo!()
    }
}

impl ForsythEdwardsNotationExt for ChessController {
    fn get_piece_placement_data() -> Vec<String> {
        todo!()
    }

    fn is_white() -> bool {
        todo!()
    }

    fn get_castling() -> String {
        todo!()
    }

    fn get_white_king_castle() -> bool {
        todo!()
    }

    fn get_white_queen_castle() -> bool {
        todo!()
    }

    fn get_black_king_castle() -> bool {
        todo!()
    }

    fn get_black_queen_castle() -> bool {
        todo!()
    }

    fn get_en_passant() -> String {
        todo!()
    }

    fn get_half_move_clock() -> u8 {
        todo!()
    }

    fn get_full_move_clock() -> u16 {
        todo!()
    }
}
