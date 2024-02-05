use api::View;

#[derive(Default, Debug, Clone)]
pub struct ChessView {}

impl ChessView {}

impl View for ChessView {
    fn get_left_board_offset(&self) -> i32 {
        todo!()
    }

    fn get_top_board_offset(&self) -> i32 {
        todo!()
    }

    fn get_square_size(&self) -> i32 {
        todo!()
    }

    fn get_width(&self) -> i32 {
        todo!()
    }

    fn get_height(&self) -> i32 {
        todo!()
    }

    fn draw_start(&self) {
        todo!()
    }

    fn draw_playing(&self) {
        todo!()
    }

    fn draw_checkmate(&self) {
        todo!()
    }

    fn draw_stalemate(&self) {
        todo!()
    }

    fn draw_resignation(&self) {
        todo!()
    }

    fn draw_draw(&self) {
        todo!()
    }

    fn draw_promotion(&self) {
        todo!()
    }

    fn draw_draw_offer(&self) {
        todo!()
    }
}
