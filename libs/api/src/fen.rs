pub trait ForsythEdwardsNotationExt {
    fn get_piece_placement_data(&self) -> Vec<String>;
    fn is_white(&self) -> bool;
    fn get_castling(&self) -> String;
    fn get_white_king_castle(&self) -> bool;
    fn get_white_queen_castle(&self) -> bool;
    fn get_black_king_castle(&self) -> bool;
    fn get_black_queen_castle(&self) -> bool;
    fn get_en_passant(&self) -> String;
    fn get_half_move_clock(&self) -> u8;
    fn get_full_move_clock(&self) -> u16;
}
