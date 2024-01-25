use crate::model::ChessModel;
use crate::view::ChessView;

use std::rc::{Rc, Weak};

pub trait Controller {
    fn set_view(&mut self, view: ChessView);
    fn set_model(&mut self, model: Option<ChessModel>);
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
}

impl Controller for ChessController {
    fn set_view(&mut self, view: ChessView) {
        self.view = Rc::downgrade(&Rc::new(view.clone()));
    }
    fn set_model(&mut self, model: Option<ChessModel>) {
        self.model = model;
    }
}
