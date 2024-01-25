use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub struct ChessModel {}

impl ChessModel {
    pub fn new() -> Self {
        ChessModel {}
    }
}

#[derive(Debug, Clone)]
pub struct ChessView {
    pub controller: Option<Rc<RefCell<ChessController>>>,
}

impl ChessView {
    pub fn new() -> Self {
        ChessView { controller: None }
    }

    pub fn set_controller(&mut self, controller: ChessController) {
        self.controller = Some(Rc::new(RefCell::new(controller)));
    }
}

#[derive(Debug, Clone)]
pub struct ChessController {
    view: Weak<ChessView>,
    model: Option<ChessModel>,
}

impl ChessController {
    pub fn new() -> Self {
        ChessController {
            view: Weak::new(),
            model: None,
        }
    }

    pub fn set_view(&mut self, view: ChessView) {
        self.view = Rc::downgrade(&Rc::new(view.clone()));
    }

    pub fn set_model(&mut self, model: Option<ChessModel>) {
        self.model = model;
    }
}

fn main() {
    let mut view = ChessView::new();
    let mut controller = ChessController::new();
    view.set_controller(controller.clone());
    controller.set_view(view.clone());
    controller.set_model(Some(ChessModel::new()));
    println!("{controller:#?}");
    println!("{view:#?}");
}
