use crate::strings::PROGRAM_NAME;
use crate::ui::GtkUi;
use epitok::auth::Auth;
use gtk::*;
use std::{cell::RefCell, rc::Rc};

pub struct App {
    pub ui: Rc<GtkUi>,
    pub auth: Rc<RefCell<Auth>>,
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
        }
    }

    pub fn connect_events(self) -> Self {
        self.connect_cancel();
        self.connect_apply();

        self
    }

    pub fn start(self) {
        self.ui.window.show_all();
        gtk::main();
    }
}

impl App {
    pub fn connect_cancel(&self) {
        let ui = self.ui.clone();
        let auth = self.auth.clone();

        self.ui.header.cancel.connect_clicked(move |_| {
            match auth.try_borrow_mut() {
                Ok(mut auth) => {
                    auth.sign_out();
                    ui.content.output.set_label("You are signed out");
                }
                Err(e) => ui.content.output.set_label(e.to_string().as_str()),
            };
        });
    }

    pub fn connect_apply(&self) {
        let ui = self.ui.clone();
        let auth = self.auth.clone();

        self.ui.header.apply.connect_clicked(move |_| {
            let new_login = ui.content.input.get_buffer().get_text();

            // if sign-in fails get error message
            match auth.try_borrow_mut() {
                Ok(mut auth) => match auth.sign_in(&new_login) {
                    Ok(()) => (),
                    Err(e) => ui.content.output.set_label(e.to_string().as_str()),
                },
                Err(e) => ui.content.output.set_label(e.to_string().as_str()),
            }

            // set label with new value
            match auth.try_borrow() {
                Ok(auth) => {
                    if let Some(login) = auth.login() {
                        ui.content.output.set_label(login)
                    }
                }
                Err(e) => ui.content.output.set_label(e.to_string().as_str()),
            }
        });
    }
}
