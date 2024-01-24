use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "draken gui";

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_ID)
        .build();
    window.present();
}

fn main() -> glib::ExitCode {
    let app = view::get_app(APP_ID);
    app.connect_activate(build_ui);
    app.run()
}
