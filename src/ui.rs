use crate::content::Content;
use crate::header::Header;
use crate::strings::PROGRAM_NAME;
use gtk::*;

pub struct GtkUi {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

impl GtkUi {
    pub fn new() -> Self {
        let window = Window::new(gtk::WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new();

        window.set_titlebar(Some(&header.container));
        window.set_title(PROGRAM_NAME);
        window.set_default_size(600, 640);
        Window::set_default_icon_name(PROGRAM_NAME);

        window.add(&content.container);

        // When application is being closed
        window.connect_delete_event(move |_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        Self {
            window,
            header,
            content,
        }
    }
}
