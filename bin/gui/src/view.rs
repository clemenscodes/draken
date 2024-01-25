use crate::controller::ChessController;
use std::{cell::RefCell, rc::Rc};

pub trait View {
    fn set_controller(&mut self, controller: ChessController);
}

#[derive(Debug, Clone)]
pub struct ChessView {
    pub controller: Option<Rc<RefCell<ChessController>>>,
}

impl ChessView {
    pub fn new() -> Self {
        ChessView { controller: None }
    }
}

impl View for ChessView {
    fn set_controller(&mut self, controller: ChessController) {
        self.controller = Some(Rc::new(RefCell::new(controller)));
    }
}
