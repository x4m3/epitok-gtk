use crate::app::App;
use epitok::auth::{Auth, Status};
use glib::clone;
use gtk::*;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

const SIGNED_IN_MSG: &str = "You are signed in as ";
const SIGN_IN_MSG: &str =
    "Sign in with your <a href=\"https://intra.epitech.eu/admin/autolog\">autologin</a> link";

fn create_status_label(auth: Rc<RefCell<Auth>>) -> Label {
    let label = Label::new(None);

    if let Ok(auth) = auth.try_borrow() {
        let text = match auth.status() {
            Status::SignedIn => match auth.login() {
                Some(login) => format!("{}<b>{}</b>", SIGNED_IN_MSG, login),
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

fn sign_in(
    action: Button,
    mut auth: RefMut<Auth>,
    status: Label,
    input: Entry,
    header_container: HeaderBar,
) {
    // Get text from input
    // If empty, attempt to get current autologin
    // If also empty, exit
    let mut input_str = input.get_buffer().get_text();
    if input_str.is_empty() {
        match auth.autologin() {
            Some(autologin) => input_str = autologin.to_string(),
            None => return,
        };
    }

    match auth.sign_in(&input_str) {
        Ok(()) => {
            match auth.login() {
                Some(login) => {
                    header_container.set_subtitle(Some(login.as_str()));
                    status.set_markup(format!("{}<b>{}</b>", SIGNED_IN_MSG, login).as_str());
                }
                None => unreachable!(),
            }
            input.hide();
            action.set_label("Sign out");
        }
        Err(e) => status.set_label(&e.to_string()),
    }
}

fn sign_out(
    action: Button,
    mut auth: RefMut<Auth>,
    status: Label,
    input: Entry,
    header_container: HeaderBar,
) {
    auth.sign_out();
    status.set_markup(SIGN_IN_MSG);
    input.show();
    action.set_label("Sign in");
    header_container.set_subtitle(None);
}

impl App {
    pub fn connect_show_settings(&self) {
        let auth = self.auth.clone();
        let ui = self.ui.clone();

        if let Ok(ui) = ui.try_borrow() {
            let header_container = ui.header.container.clone();
            let parent_window = ui.window.clone();

            ui.header.settings.connect_clicked(move |_| {
                let window = Window::new(WindowType::Toplevel);
                window.set_title("Settings");
                window.set_default_size(525, 300);
                window.set_transient_for(Some(&parent_window)); // Set which window is the parent
                window.set_position(WindowPosition::CenterOnParent); // Settings window will open in center of main window
                // window.set_modal(true); // Can't interact with parent window
                window.set_destroy_with_parent(true); // Destroy window if main window is destroyed

                let container = Box::new(Orientation::Vertical, 0);
                let status = create_status_label(auth.clone());
                let action = create_action_button(auth.clone());

                let input = Entry::new();
                input.set_alignment(0.5); // Center content

                let account = Label::new(None);
                account.set_markup("<span size=\"large\">Account</span>");

                container.pack_start(&account, true, false, 10);
                container.pack_start(&status, true, false, 0);
                container.pack_start(&input, true, false, 0);
                container.pack_end(&action, true, false, 10);

                window.add(&container);
                window.show_all();

                // If we are already signed in don't show entry
                if let Ok(auth) = auth.try_borrow() {
                    if let Status::SignedIn = auth.status() { input.hide() }
                }

                // When action button is clicked
                action.connect_clicked(clone!(@weak action, @weak auth, @weak header_container => move |_| {
                if let Ok(auth) = auth.try_borrow_mut() {
                    match auth.status() {
                        Status::SignedIn => sign_out(action, auth, status.clone(), input.clone(), header_container),
                        _ => sign_in(action, auth, status.clone(), input.clone(), header_container),
                    }
                };
            }));
            });
        };
    }
}
