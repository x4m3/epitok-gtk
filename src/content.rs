use epitok::{
    auth::Auth,
    event::{list_events, list_events_today, Event},
};
use gtk::*;
use std::cell::RefCell;

pub struct Content {
    pub container: Box,
    pub events: Events,
    pub students: Students,
}

pub struct Events {
    pub container: Box,
    pub list_box: ListBox,
    pub list_box_rows: Vec<ListBoxRow>,
}

pub struct Students {
    pub container: Box,
    pub list_box: ListBox,
    pub list_box_rows: Vec<ListBoxRow>,
    pub action_bar: ActionBar,
    pub start_scan: Button,
    pub save: Button,
    pub reset: Button,
    pub set_remaining_missing: Button,
}

impl Content {
    pub fn new() -> Self {
        let container_orientation = Orientation::Vertical;
        let container = Box::new(container_orientation, 0);
        let events = Events::new();
        let separator = Separator::new(container_orientation);
        let students = Students::new();

        container.pack_start(&events.container, true, true, 0);
        container.pack_start(&separator, false, false, 0);
        container.pack_end(&students.container, true, true, 0);

        Self {
            container,
            events,
            students,
        }
    }
}

impl Events {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 0);
        let scrolled_window = ScrolledWindow::new::<Adjustment, Adjustment>(None, None);
        let list_box = ListBox::new();
        let list_box_rows: Vec<ListBoxRow> = Vec::new();

        // let mut vec_strings: Vec<String> = Vec::new();
        // vec_strings
        //     .push("<b>14:00 - 14:30</b>: B2 - Unix System programming - KickOff - Navy".into());
        // vec_strings
        //     .push("<b>14:00 - 14:30</b>: B2 - Unix System programming - Unit Kick-off".into());
        // vec_strings
        //     .push("<b>14:30 - 17:30</b>: B2 - Unix System programming - Bootstrap - Navy".into());

        // for event in vec_strings {
        //     let list_box_row = ListBoxRow::new();
        //     let list_box_row_box = Box::new(Orientation::Horizontal, 0);
        //     let label = Label::new(None);
        //     label.set_markup(event.as_str());
        //     list_box_row_box.add(&label);
        //     list_box_row.add(&list_box_row_box);
        //     list_box.add(&list_box_row);
        //
        //     list_box_rows.push(list_box_row);
        // }

        scrolled_window.add(&list_box);
        container.pack_start(&scrolled_window, true, true, 0);

        Self {
            container,
            list_box,
            list_box_rows,
        }
    }

    pub fn populate(&mut self, events: &[Event]) {
        if !self.list_box_rows.is_empty() {
            // TODO: Destroy every gtk element

            // Clear vector
            self.list_box_rows.clear();
        }

        // Add events
        for event in events {
            let list_box_row = ListBoxRow::new();
            let list_box_row_box = Box::new(Orientation::Horizontal, 0);
            let label = Label::new(None);
            let formatted_module = html_escape::encode_safe(event.module());
            let formatted_title = html_escape::encode_safe(event.title());
            let label_str = format!(
                "<b>{} - {}</b>: {} - {}",
                event.start(),
                event.end(),
                formatted_module,
                formatted_title,
            );
            println!("{}", label_str);
            label.set_markup(label_str.as_str());
            list_box_row_box.add(&label);
            list_box_row.add(&list_box_row_box);
            self.list_box.add(&list_box_row);

            self.list_box_rows.push(list_box_row);
        }

        self.list_box.show_all();
    }
}

impl Students {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 0);
        let scrolled_window = ScrolledWindow::new::<Adjustment, Adjustment>(None, None);
        let list_box = ListBox::new();
        let list_box_rows: Vec<ListBoxRow> = Vec::new();

        // let mut vec_strings: Vec<String> = Vec::new();
        // vec_strings.push("philippe.loctaux@epitech.eu".into());
        // vec_strings.push("theo.boscher@epitech.eu".into());
        // vec_strings.push("francois.lelay@epitech.eu".into());
        // vec_strings.push("alexandre1.wagner@epitech.eu".into());
        //
        // for student in vec_strings {
        //     let list_box_row = ListBoxRow::new();
        //     let list_box_row_box = Box::new(Orientation::Horizontal, 0);
        //     let label_student = Label::new(Some(student.as_str()));
        //     list_box_row_box.pack_start(&label_student, false, false, 0);
        //     let button1_1 = ToggleButton::with_label("Present");
        //     list_box_row_box.pack_end(&button1_1, false, false, 0);
        //     let button1_2 = ToggleButton::with_label("Missing");
        //     list_box_row_box.pack_end(&button1_2, false, false, 0);
        //     let button1_3 = ToggleButton::with_label("N/A");
        //     list_box_row_box.pack_end(&button1_3, false, false, 0);
        //     let button1_4 = ToggleButton::with_label("None");
        //     button1_4.set_active(true);
        //     button1_4.set_sensitive(false);
        //     list_box_row_box.pack_end(&button1_4, false, false, 0);
        //     list_box_row.add(&list_box_row_box);
        //     list_box.add(&list_box_row);
        //
        //     list_box_rows.push(list_box_row);
        // }

        let action_bar = ActionBar::new();

        let reset = Button::with_label("Reset");
        reset.get_style_context().add_class("destructive-action");
        action_bar.pack_start(&reset);

        let start_scan = Button::with_label("Start scanning");
        action_bar.pack_start(&start_scan);

        let save = Button::with_label("Save");
        save.get_style_context().add_class("suggested-action");
        action_bar.pack_end(&save);

        let set_remaining_missing = Button::with_label("Set remaining as missing");
        action_bar.pack_end(&set_remaining_missing);

        scrolled_window.add(&list_box);
        container.pack_start(&scrolled_window, true, true, 0);
        container.pack_end(&action_bar, false, false, 0);

        Self {
            container,
            list_box,
            list_box_rows,
            action_bar,
            start_scan,
            save,
            reset,
            set_remaining_missing,
        }
    }
}

pub fn get_events(auth: &RefCell<Auth>, events: &RefCell<Vec<Event>>, content: &RefCell<Content>) {
    if let Ok(auth) = auth.try_borrow() {
        if let Ok(mut events) = events.try_borrow_mut() {
            // match list_events_today(&mut events, auth.autologin()) { // TODO: use today and not hardcoded date
            match list_events(&mut events, auth.autologin(), "2020-02-18") {
                Ok(_) => {
                    if let Ok(mut content) = content.try_borrow_mut() {
                        content.events.populate(&events);
                    }
                }
                Err(e) => eprintln!("error while getting events: {}", e),
            }
        }
    }
}
