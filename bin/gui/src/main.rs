mod controller;
mod model;
mod view;

use controller::*;
use model::*;
use view::*;

fn main() {
    let model = ChessModel::new();
    let mut view = ChessView::new();
    let mut controller = ChessController::new();
    controller.set_view(view.clone());
    controller.set_model(Some(model.clone()));
    view.set_controller(controller.clone());
    println!("{model:#?}");
    println!("{controller:#?}");
    println!("{view:#?}");
}
