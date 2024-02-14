use api::{Controller, ForsythEdwardsNotationExt, GameExt, Model, Square};
use model::ChessModel;
use std::error::Error;

#[derive(Default, Debug, Clone)]
pub struct ChessController {
    model: ChessModel,
}

impl ChessController {
    pub fn model(&self) -> ChessModel {
        self.model
    }

    pub fn model_mut(&mut self) -> &mut ChessModel {
        &mut self.model
    }

    pub fn set_model(&mut self, model: ChessModel) {
        self.model = model;
    }
}

impl Controller for ChessController {
    fn handle_mouse_pressed(&self) {
        todo!()
    }

    fn handle_mouse_dragged(&self) {
        todo!()
    }

    fn handle_mouse_released(&self) {
        todo!()
    }

    fn handle_mouse_moved(&self) {
        todo!()
    }

    fn get_error_message(&self) -> String {
        todo!()
    }

    fn clear_error_message(&mut self) {
        self.model_mut().clear_error()
    }

    fn get_fen(&self) -> String {
        self.model().get_fen()
    }

    fn get_source(&self) -> Square {
        todo!()
    }

    fn get_destination(&self) -> Square {
        todo!()
    }

    fn get_dragged_square(&self) -> Square {
        todo!()
    }

    fn get_legal_moves(&self) -> Vec<(Square, Square, Option<char>)> {
        self.model().get_legal_moves(self.get_source())
    }
}

impl GameExt for ChessController {
    fn init_game(&mut self) {
        self.model_mut().init_game()
    }

    fn start_game(&mut self) {
        self.model_mut().start_game()
    }

    fn resign(&mut self) {
        self.model_mut().resign()
    }

    fn offer_draw(&self) {
        self.model().offer_draw()
    }

    fn accept_draw(&mut self) {
        self.model_mut().accept_draw()
    }

    fn decline_draw(&self) {
        self.model().decline_draw()
    }

    fn claim_draw(&mut self) {
        self.model_mut().claim_draw()
    }

    fn get_game_state(&self) {
        self.model().get_game_state()
    }

    fn get_moves(&self) {
        self.model().get_moves()
    }

    fn promote_queen(&mut self) {
        self.model_mut().promote_queen()
    }

    fn promote_rook(&mut self) {
        self.model_mut().promote_rook()
    }

    fn promote_knight(&mut self) {
        self.model_mut().promote_knight()
    }

    fn promote_bishop(&mut self) {
        self.model_mut().promote_bishop()
    }

    fn is_own_piece_on_square(&self, square: Square) -> bool {
        self.model().is_own_piece_on_square(square)
    }

    fn make_move(&mut self, source: Square, destination: Square, promotion: Option<char>) -> Result<(), Box<dyn Error>> {
        self.model_mut().make_move(source, destination, promotion)
    }

    fn ply(&self) -> u16 {
        self.model().ply()
    }
}

impl ForsythEdwardsNotationExt for ChessController {
    fn get_piece_placement_data(&self) -> Vec<String> {
        self.model().get_piece_placement_data()
    }

    fn is_white(&self) -> bool {
        self.model().is_white()
    }

    fn get_castling(&self) -> String {
        self.model().get_castling()
    }

    fn get_white_king_castle(&self) -> bool {
        self.model().get_white_king_castle()
    }

    fn get_white_queen_castle(&self) -> bool {
        self.model().get_white_queen_castle()
    }

    fn get_black_king_castle(&self) -> bool {
        self.model().get_black_king_castle()
    }

    fn get_black_queen_castle(&self) -> bool {
        self.model().get_black_queen_castle()
    }

    fn get_en_passant(&self) -> String {
        self.model().get_en_passant()
    }

    fn get_half_move_clock(&self) -> u8 {
        self.model().get_half_move_clock()
    }

    fn get_full_move_clock(&self) -> u16 {
        self.model().get_full_move_clock()
    }
}
