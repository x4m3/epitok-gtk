use gtk::*;

pub struct Content {
    pub container: Box,
    pub events: Events,
    pub students: Students,
}

pub struct Events {
    pub container: Box,
}

pub struct Students {
    pub container: Box,
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

        let list_box_row = ListBoxRow::new();
        let list_box_row_box = Box::new(Orientation::Horizontal, 0);
        let label = Label::new(None);
        label.set_markup("<b>14:00 - 14:30</b>: B2 - Unix System programming - KickOff - Navy");
        list_box_row_box.add(&label);
        list_box_row.add(&list_box_row_box);
        list_box.add(&list_box_row);

        let list_box_row1 = ListBoxRow::new();
        let list_box_row_box1 = Box::new(Orientation::Horizontal, 0);
        let label1 = Label::new(None);
        label1.set_markup("<b>14:00 - 14:30</b>: B2 - Unix System programming - Unit Kick-off");
        list_box_row_box1.add(&label1);
        list_box_row1.add(&list_box_row_box1);
        list_box.add(&list_box_row1);

        let list_box_row2 = ListBoxRow::new();
        let list_box_row_box2 = Box::new(Orientation::Horizontal, 0);
        let label2 = Label::new(None);
        label2.set_markup("<b>14:30 - 17:30</b>: B2 - Unix System programming - Bootstrap - Navy");
        list_box_row_box2.add(&label2);
        list_box_row2.add(&list_box_row_box2);
        list_box.add(&list_box_row2);

        scrolled_window.add(&list_box);
        container.pack_start(&scrolled_window, true, true, 0);

        Self { container }
    }
}

impl Students {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 0);
        let scrolled_window = ScrolledWindow::new::<Adjustment, Adjustment>(None, None);
        let list_box = ListBox::new();

        let list_box_row1 = ListBoxRow::new();
        let list_box_row_box1 = Box::new(Orientation::Horizontal, 0);
        let label1 = Label::new(None);
        label1.set_label("philippe.loctaux@epitech.eu");
        list_box_row_box1.pack_start(&label1, false, false, 0);
        let button1_1 = ToggleButton::with_label("Present");
        list_box_row_box1.pack_end(&button1_1, false, false, 0);
        let button1_2 = ToggleButton::with_label("Missing");
        list_box_row_box1.pack_end(&button1_2, false, false, 0);
        let button1_3 = ToggleButton::with_label("N/A");
        list_box_row_box1.pack_end(&button1_3, false, false, 0);
        let button1_4 = ToggleButton::with_label("None");
        button1_4.set_active(true);
        button1_4.set_sensitive(false);
        list_box_row_box1.pack_end(&button1_4, false, false, 0);
        list_box_row1.add(&list_box_row_box1);
        list_box.add(&list_box_row1);

        let list_box_row2 = ListBoxRow::new();
        let list_box_row_box2 = Box::new(Orientation::Horizontal, 0);
        let label2 = Label::new(None);
        label2.set_label("theo.boscher@epitech.eu");
        list_box_row_box2.add(&label2);
        let button2_1 = ToggleButton::with_label("Present");
        list_box_row_box2.pack_end(&button2_1, false, false, 0);
        let button2_2 = ToggleButton::with_label("Missing");
        list_box_row_box2.pack_end(&button2_2, false, false, 0);
        let button2_3 = ToggleButton::with_label("N/A");
        list_box_row_box2.pack_end(&button2_3, false, false, 0);
        let button2_4 = ToggleButton::with_label("None");
        button2_4.set_active(true);
        button2_4.set_sensitive(false);
        list_box_row_box2.pack_end(&button2_4, false, false, 0);
        list_box_row2.add(&list_box_row_box2);
        list_box.add(&list_box_row2);

        let list_box_row3 = ListBoxRow::new();
        let list_box_row_box3 = Box::new(Orientation::Horizontal, 0);
        let label3 = Label::new(None);
        label3.set_label("francois.lelay@epitech.eu");
        list_box_row_box3.add(&label3);
        let button3_1 = ToggleButton::with_label("Present");
        list_box_row_box3.pack_end(&button3_1, false, false, 0);
        let button3_2 = ToggleButton::with_label("Missing");
        list_box_row_box3.pack_end(&button3_2, false, false, 0);
        let button3_3 = ToggleButton::with_label("N/A");
        list_box_row_box3.pack_end(&button3_3, false, false, 0);
        let button3_4 = ToggleButton::with_label("None");
        button3_4.set_active(true);
        button3_4.set_sensitive(false);
        list_box_row_box3.pack_end(&button3_4, false, false, 0);
        list_box_row3.add(&list_box_row_box3);
        list_box.add(&list_box_row3);

        let list_box_row4 = ListBoxRow::new();
        let list_box_row_box4 = Box::new(Orientation::Horizontal, 0);
        let label4 = Label::new(None);
        label4.set_label("alexandre1.wagner@epitech.eu");
        list_box_row_box4.add(&label4);
        let button4_1 = ToggleButton::with_label("Present");
        list_box_row_box4.pack_end(&button4_1, false, false, 0);
        let button4_2 = ToggleButton::with_label("Missing");
        list_box_row_box4.pack_end(&button4_2, false, false, 0);
        let button4_3 = ToggleButton::with_label("N/A");
        list_box_row_box4.pack_end(&button4_3, false, false, 0);
        let button4_4 = ToggleButton::with_label("None");
        button4_4.set_active(true);
        button4_4.set_sensitive(false);
        list_box_row_box4.pack_end(&button4_4, false, false, 0);
        list_box_row4.add(&list_box_row_box4);
        list_box.add(&list_box_row4);

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
            action_bar,
            start_scan,
            save,
            reset,
            set_remaining_missing,
        }
    }
}
