use gio::prelude::*;
use gtk::prelude::*;

const PROGRAM_NAME: &str = "epitok";
const APPLICATION_ID: &str = "com.philippeloctaux.epitok";

fn build_ui(app: &gtk::Application) {
    let application_window = gtk::ApplicationWindow::new(app);

    application_window.set_title(PROGRAM_NAME);
    application_window.set_border_width(10);
    application_window.set_position(gtk::WindowPosition::Center);
    application_window.set_default_size(1024, 600);

    application_window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some(APPLICATION_ID), Default::default())
        .expect("Failed to initialize gtk application");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&std::env::args().collect::<Vec<_>>());
}
