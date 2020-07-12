use gtk::*;

pub struct Content {
    pub container: Box,
    pub input: Entry,
    pub output: Label,
}

impl Content {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 0);

        let box_current_login = Box::new(Orientation::Horizontal, 0);
        let label_current_login = Label::new("Current login: ".into());

        let label_login = Label::new("Please sign-in".into());

        box_current_login.set_halign(Align::Center);
        label_current_login.set_halign(Align::Start);
        label_login.set_halign(Align::Start);

        box_current_login.pack_start(&label_current_login, false, false, 5);
        box_current_login.pack_start(&label_login, true, true, 5);

        let input_login = Entry::new();

        container.pack_start(&box_current_login, true, false, 0);
        container.pack_start(&Separator::new(Orientation::Horizontal), false, false, 0);
        container.pack_start(&input_login, true, false, 0);

        Self {
            container,
            input: input_login,
            output: label_login,
        }
    }
}
