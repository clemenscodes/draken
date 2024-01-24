use gtk::Application;

pub fn get_app(app_id: &str) -> Application {
    Application::builder().application_id(app_id).build()
}
