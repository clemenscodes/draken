use controller::ChessController;
use model::ChessModel;
use view::ChessView;

fn main() {
    let model = ChessModel::default();
    let view = ChessView::default();
    let controller = ChessController::default();
    println!("{model:#?}");
    println!("{controller:#?}");
    println!("{view:#?}");
}
