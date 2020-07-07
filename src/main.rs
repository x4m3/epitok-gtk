use gtk::*;

const PROGRAM_NAME: &str = "epitok";

pub struct App {
    pub window: Window,
    pub header: Header,
}

pub struct Header {
    pub container: HeaderBar,
}

impl App {
    fn new() -> App {
        let window = Window::new(gtk::WindowType::Toplevel);
        let header = Header::new();

        window.set_titlebar(Some(&header.container));
        window.set_title(PROGRAM_NAME);
        Window::set_default_icon_name(PROGRAM_NAME);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App { window, header }
    }
}

impl Header {
    fn new() -> Header {
        let container = HeaderBar::new();

        container.set_title(Some(PROGRAM_NAME));
        container.set_show_close_button(true);

        Header { container }
    }
}

fn main() {
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK application");
        std::process::exit(1);
    }

    let app = App::new();
    app.window.show_all();

    gtk::main();
}
