use epitok::{
    auth::Auth,
    event::{list_events, Event},
    student::Presence,
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

    pub fn populate(&mut self, events: &mut [Event]) {
        // Clean events and students
        clear_content(&self.events.list_box, &mut self.events.list_box_rows);
        clear_content(&self.students.list_box, &mut self.students.list_box_rows);

        self.events.populate(&events);
        for event in events.iter_mut() {
            self.students.populate(event);
        }
    }
}

impl Events {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 0);
        let scrolled_window = ScrolledWindow::new::<Adjustment, Adjustment>(None, None);
        let list_box = ListBox::new();
        let list_box_rows: Vec<ListBoxRow> = Vec::new();

        scrolled_window.add(&list_box);
        container.pack_start(&scrolled_window, true, true, 0);

        Self {
            container,
            list_box,
            list_box_rows,
        }
    }

    pub fn populate(&mut self, events: &[Event]) {
        // Add new events
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
            label.set_markup(label_str.as_str());
            list_box_row_box.add(&label);
            list_box_row.add(&list_box_row_box);
            self.list_box.add(&list_box_row);

            self.list_box_rows.push(list_box_row);
        }

        // Display new events
        self.list_box.show_all();
    }
}

impl Students {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 0);
        let scrolled_window = ScrolledWindow::new::<Adjustment, Adjustment>(None, None);
        let list_box = ListBox::new();
        let list_box_rows: Vec<ListBoxRow> = Vec::new();

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

    pub fn populate(&mut self, event: &mut Event) {
        // Add new students
        for student in event.students() {
            let list_box_row = ListBoxRow::new();
            let list_box_row_box = Box::new(Orientation::Horizontal, 0);

            let label_student = Label::new(Some(student.get_login()));
            list_box_row_box.pack_start(&label_student, false, false, 0);

            let button_present = ToggleButton::with_label("Present");
            list_box_row_box.pack_end(&button_present, false, false, 0);

            let button_missing = ToggleButton::with_label("Missing");
            list_box_row_box.pack_end(&button_missing, false, false, 0);

            let button_not_applicable = ToggleButton::with_label("N/A");
            list_box_row_box.pack_end(&button_not_applicable, false, false, 0);

            let button_none = ToggleButton::with_label("None");
            list_box_row_box.pack_end(&button_none, false, false, 0);

            // If student is set as `failed`, set it as `missing`
            // I don't want to have a `failed` button since it should be in use
            // But just in case
            if let Presence::Failed = student.get_presence() {
                student.set_presence(Presence::Missing);
            }

            // Mark current status button as locked
            // aka you can't select it again
            let current_status = match student.get_presence() {
                Presence::Present => button_present,
                Presence::Missing => button_missing,
                Presence::NotApplicable => button_not_applicable,
                Presence::None => button_none,
                Presence::Failed => button_missing,
            };
            current_status.set_active(true);
            current_status.set_sensitive(false);

            list_box_row.add(&list_box_row_box);
            self.list_box.add(&list_box_row);

            self.list_box_rows.push(list_box_row);
        }

        // Display new students
        self.list_box.show_all();
    }
}

pub fn get_events(auth: &RefCell<Auth>, events: &RefCell<Vec<Event>>, content: &RefCell<Content>) {
    println!("Fetching events...");
    if let Ok(auth) = auth.try_borrow() {
        if let Ok(mut events) = events.try_borrow_mut() {
            // match list_events_today(&mut events, auth.autologin()) { // TODO: use today and not hardcoded date
            match list_events(&mut events, auth.autologin(), "2020-06-15") {
                Ok(_) => {
                    if let Ok(mut content) = content.try_borrow_mut() {
                        content.populate(&mut events);
                    }
                }
                Err(e) => eprintln!("Error while getting events: {}", e),
            }
        }
    }
    println!("Events fetched");
}

fn clear_content(list_box: &ListBox, list_box_rows: &mut Vec<ListBoxRow>) {
    // Destroy every gtk element
    for list_box_row in list_box_rows.iter_mut() {
        list_box.remove(list_box_row);
    }

    // Clear vector
    list_box_rows.clear();
}
