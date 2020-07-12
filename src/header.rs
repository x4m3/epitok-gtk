use crate::strings::PROGRAM_NAME;
use gtk::*;

pub struct Header {
    pub container: HeaderBar,
    pub cancel: Button,
    pub apply: Button,
}

impl Header {
    pub fn new() -> Self {
        let container = HeaderBar::new();

        container.set_title(Some(PROGRAM_NAME));
        container.set_show_close_button(true);

        let cancel = Button::with_label("Cancel");
        let apply = Button::with_label("Apply");

        cancel.get_style_context().add_class("destructive-action");
        apply.get_style_context().add_class("suggested-action");

        container.pack_start(&cancel);
        container.pack_end(&apply);

        Self {
            container,
            cancel,
            apply,
        }
    }
}
