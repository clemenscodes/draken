use controller::ChessController;
use model::ChessModel;
use view::ChessView;

fn main() {
    let model = ChessModel::new();
    let view = ChessView::new();
    let controller = ChessController::new();
    println!("{model:#?}");
    println!("{controller:#?}");
    println!("{view:#?}");
}
