use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Model {}

#[derive(Debug)]
pub struct View {
    pub controller: Rc<Controller>,
}

#[derive(Debug)]
pub struct Controller {
    view: Weak<View>,
    model: Model,
}

impl Controller {
    fn set_view(&mut self, view: Weak<View>) {
        self.view = view;
    }
    fn set_model(&mut self, model: Model) {
        self.model = model;
    }
}

impl View {
    fn set_controller(&mut self, controller: Rc<Controller>) {
        self.controller = controller;
    }
}

fn main() {
    let mut view = Rc::new_cyclic(|view| View {
        controller: Rc::new(Controller {
            view: view.clone(),
            model: Model {},
        }),
    });
    let mut controller = view.controller.clone();
    println!("{controller:#?}");
    println!("{view:#?}");
}
