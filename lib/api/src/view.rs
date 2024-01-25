pub trait View {
    fn get_left_board_offset(&self) -> i32;
    fn get_top_board_offset(&self) -> i32;
    fn get_square_size(&self) -> i32;
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
    fn draw_start(&self);
    fn draw_playing(&self);
    fn draw_checkmate(&self);
    fn draw_stalemate(&self);
    fn draw_resignation(&self);
    fn draw_draw(&self);
    fn draw_promotion(&self);
    fn draw_draw_offer(&self);
}
