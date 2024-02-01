use controller::ChessController;
use model::ChessModel;
use view::ChessView;

fn main() {
    let model = ChessModel::default();
    let view = ChessView::new();
    let controller = ChessController::new();
    println!("{model:#?}");
    println!("{controller:#?}");
    println!("{view:#?}");
}
