use crate::content::Content;
use crate::storage::Storage;
use crate::strings::PROGRAM_NAME;
use crate::ui::GtkUi;
use epitok::{auth::Auth, event::Event};
use gtk::*;
use std::{cell::RefCell, rc::Rc};

pub struct App {
    pub ui: Rc<RefCell<GtkUi>>,
    pub auth: Rc<RefCell<Auth>>,
    pub events: Rc<RefCell<Vec<Event>>>,
    pub storage: Storage,
    pub content: Rc<RefCell<Content>>,
}

impl App {
    pub fn new() -> Self {
        glib::set_program_name(PROGRAM_NAME.into());

        if gtk::init().is_err() {
            eprintln!("Failed to initialize GTK application");
            std::process::exit(1);
        }

        let auth = Rc::new(RefCell::new(Auth::new()));
        let events = Rc::new(RefCell::new(Vec::new()));
        let storage = Self::try_load_config(&auth);
        let content = Rc::new(RefCell::new(Content::new()));
        let ui = Rc::new(RefCell::new(GtkUi::new(&auth, &content)));

        Self {
            auth,
            events,
            ui,
            storage,
            content,
        }
    }

    fn try_load_config(auth: &Rc<RefCell<Auth>>) -> Storage {
        // Attempt to load configuration
        // If attempt fails, return an empty configuration
        let storage = match Storage::load() {
            Ok(new) => new,
            Err(e) => {
                eprintln!("Failed to load configuration: {}", e);
                Storage::new()
            }
        };

        // Configuration has been loaded
        // Attempt to sign in
        let autologin = storage.autologin.clone();
        if let Some(autologin) = autologin {
            if let Ok(mut auth) = auth.try_borrow_mut() {
                match auth.sign_in(&autologin) {
                    Ok(()) => (),
                    Err(e) => eprintln!("error when signing in: {}", e),
                }
            }
        };

        storage
    }

    pub fn connect_events(self) -> Self {
        self.connect_refresh_event();
        self.connect_show_about();
        self.connect_show_settings();

        self
    }

    pub fn try_get_events(self) -> Self {
        let auth = self.auth.clone();
        let events = self.events.clone();
        let content = self.content.clone();
        crate::content::get_events(&auth, &events, &content);

        self
    }

    pub fn start(self) -> Self {
        if let Ok(ui) = self.ui.try_borrow() {
            ui.window.show_all();
            gtk::main();
        }

        self
    }

    pub fn save_settings(&mut self) {
        // Get current autologin
        if let Ok(auth) = self.auth.try_borrow() {
            self.storage.autologin = auth.autologin().to_owned();
        }

        // Save configuration
        if let Err(e) = self.storage.save() {
            eprintln!("failed to save configuration: {}", e);
        }
    }
}
