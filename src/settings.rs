use crate::app::App;
use epitok::auth::Auth;
use glib::clone;
use gtk::*;
use std::{cell::RefCell, rc::Rc};

fn create_action_button(auth: Rc<RefCell<Auth>>) -> Button {
    let button = Button::new();

    if let Ok(auth) = auth.try_borrow() {
        match auth.status() {
            true => {
                button.set_label("Sign out");
                button.get_style_context().add_class("destructive-action");
            }
            false => {
                button.set_label("Sign in");
                button.get_style_context().add_class("suggested-action");
            }
        }
    }

    button
}

impl App {
    pub fn connect_show_settings(&self) {
        let auth = self.auth.clone();

        self.ui.header.settings.connect_clicked(move |_| {
            let window = Window::new(WindowType::Toplevel);
            window.set_title("Settings");
            window.set_default_size(340, 300);

            let container = Box::new(Orientation::Vertical, 0);
            let action = create_action_button(auth.clone());

            container.pack_start(&action, true, false, 0);

            window.add(&container);

            action.connect_clicked(clone!(@weak action => move |_| {
                action.set_label("this is a test");
            }));

            window.show_all();
        });
    }
}
