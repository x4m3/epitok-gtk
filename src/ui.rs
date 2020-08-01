use crate::content::Content;
use crate::header::Header;
use crate::strings::{PROGRAM_ID, PROGRAM_NAME};
use epitok::auth::Auth;
use gtk::*;
use std::cell::RefCell;

pub struct GtkUi {
    pub window: Window,
    pub header: Header,
}

impl GtkUi {
    pub fn new(auth: &RefCell<Auth>, content: &RefCell<Content>) -> Self {
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new(auth);

        window.set_titlebar(Some(&header.container));
        window.set_title(PROGRAM_NAME);
        window.set_default_size(600, 640);
        window.set_icon_name(PROGRAM_ID.into());

        if let Ok(content) = content.try_borrow() {
            window.add(&content.container);
        }

        // When application is being closed
        window.connect_delete_event(move |_, _| {
            println!("Closing window");
            gtk::main_quit();
            Inhibit(false)
        });

        Self { window, header }
    }
}
