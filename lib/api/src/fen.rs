pub trait ForsythEdwardsNotationExt {
    fn get_piece_placement_data() -> Vec<String>;
    fn is_white() -> bool;
    fn get_castling() -> String;
    fn get_white_king_castle() -> bool;
    fn get_white_queen_castle() -> bool;
    fn get_black_king_castle() -> bool;
    fn get_black_queen_castle() -> bool;
    fn get_en_passant() -> String;
    fn get_half_move_clock() -> u8;
    fn get_full_move_clock() -> u16;
}
