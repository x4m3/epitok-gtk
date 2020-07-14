use crate::strings::PROGRAM_NAME;
use crate::ui::GtkUi;
use epitok::{auth::Auth, event::Event};
use gtk::*;
use std::{cell::RefCell, rc::Rc};

pub struct App {
    pub ui: Rc<GtkUi>,
    pub auth: Rc<RefCell<Auth>>,
    pub events: Rc<RefCell<Vec<Event>>>,
}

impl App {
    pub fn new() -> Self {
        glib::set_program_name(PROGRAM_NAME.into());

        if gtk::init().is_err() {
            eprintln!("Failed to initialize GTK application");
            std::process::exit(1);
        }

        Self {
            ui: Rc::new(GtkUi::new()),
            auth: Rc::new(RefCell::new(Auth::new())),
            events: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn connect_events(self) -> Self {
        self.connect_refresh_event();
        self
    }

    pub fn start(self) {
        self.ui.window.show_all();
        gtk::main();
    }
}
