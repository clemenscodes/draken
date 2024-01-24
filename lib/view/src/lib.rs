use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

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
    let window = ApplicationWindow::builder()
        .application(app)
        .title(title)
        .build();
    window.present();
}
