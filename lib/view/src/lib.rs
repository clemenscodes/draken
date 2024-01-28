use api::View;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

#[derive(Debug, Clone)]
pub struct ChessView {}

impl ChessView {
    pub fn new() -> Self {
        ChessView {}
    }
}

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

pub fn run(app_id: &str) -> i32 {
    let app = get_app(app_id);
    app.connect_activate(build_ui);
    let exitcode = app.run();
    exitcode.value()
}

fn get_app(app_id: &str) -> Application {
    Application::builder().application_id(app_id).build()
}

fn build_ui(app: &Application) {
    let title = "draken";
    let window = ApplicationWindow::builder().application(app).title(title).build();
    window.present();
}
