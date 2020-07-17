use crate::app::App;
use epitok::auth::{Auth, Status};
use glib::clone;
use gtk::*;
use std::{cell::RefCell, rc::Rc};

const SIGNED_IN_MSG: &str = "You are signed in as ";
const SIGN_IN_MSG: &str =
    "Sign in with your <a href=\"https://intra.epitech.eu/admin/autolog\">autologin</a> link";

fn create_status_label(auth: Rc<RefCell<Auth>>) -> Label {
    let label = Label::new(None);

    if let Ok(auth) = auth.try_borrow() {
        let text = match auth.status() {
            Status::SignedIn => match auth.login() {
                Some(login) => format!("{}{}", SIGNED_IN_MSG, login),
                None => unreachable!(),
            },
            Status::SignedOut => SIGN_IN_MSG.to_string(),
            Status::Error(e) => e.to_string(),
        };
        label.set_markup(&text);
    }
    label
}

fn create_action_button(auth: Rc<RefCell<Auth>>) -> Button {
    let button = Button::new();

    if let Ok(auth) = auth.try_borrow() {
        match auth.status() {
            Status::SignedIn => button.set_label("Sign out"),
            _ => button.set_label("Sign in"),
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
            let account = Label::new("Account".into());
            let status = create_status_label(auth.clone());
            let action = create_action_button(auth.clone());
            let input = Entry::new();

            container.pack_start(&account, false, false, 0);
            container.pack_start(&status, false, false, 0);
            container.pack_start(&input, false, false, 0);
            container.pack_start(&action, false, false, 0);

            window.add(&container);

            action.connect_clicked(
                clone!(@weak action, @weak auth, @weak status, @weak input => move |_| {
                        if let Ok(mut auth) = auth.try_borrow_mut() {
                            match auth.status() {
                                Status::SignedIn => {
                                    auth.sign_out();
                                    status.set_markup(SIGN_IN_MSG);
                                    input.show();
                                    action.set_label("Sign in");
                                },
                                _ => {
                                let input_str = input.get_buffer().get_text();
                                match auth.sign_in(&input_str) {
                                Ok(()) => {
                match auth.login() {
                    Some(login) => status.set_label(format!("{}{}", SIGNED_IN_MSG, login).as_str()),
                    None => unreachable!(),
                }
                                    input.hide();
                                    action.set_label("Sign out");
                                },
                                Err(e) => status.set_label(&e.to_string()),
                                }
                                },
                            }
                        };
                    }),
            );

            window.show_all();
        });
    }
}
