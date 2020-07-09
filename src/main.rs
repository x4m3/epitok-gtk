use epitok::auth::Auth;
use gtk::*;
use std::{cell::RefCell, rc::Rc};

const PROGRAM_NAME: &str = "epitok";

pub struct Data {
    pub auth: Auth,
}

pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

pub struct Header {
    pub container: HeaderBar,
    pub cancel: Button,
    pub apply: Button,
}

pub struct Content {
    pub container: Box,
    pub input: Entry,
    pub output: Label,
}

impl Data {
    fn new() -> Data {
        Data { auth: Auth::new() }
    }
}

impl App {
    fn new(data: &Data) -> App {
        let window = Window::new(gtk::WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new(data);

        window.set_titlebar(Some(&header.container));
        window.set_title(PROGRAM_NAME);
        Window::set_default_icon_name(PROGRAM_NAME);

        window.add(&content.container);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App {
            window,
            header,
            content,
        }
    }
}

impl Header {
    fn new() -> Header {
        let container = HeaderBar::new();

        container.set_title(Some(PROGRAM_NAME));
        container.set_show_close_button(true);

        let cancel = Button::with_label("Cancel");
        let apply = Button::with_label("Apply");

        cancel.get_style_context().add_class("destructive-action");
        apply.get_style_context().add_class("suggested-action");

        container.pack_start(&cancel);
        container.pack_end(&apply);

        Header {
            container,
            cancel,
            apply,
        }
    }
}

impl Content {
    fn new(data: &Data) -> Content {
        let container = Box::new(Orientation::Vertical, 0);

        let box_current_login = Box::new(Orientation::Horizontal, 0);
        let label_current_login = Label::new("Current login: ".into());

        let label_login = Label::new(data.auth.login().to_owned().as_deref());

        box_current_login.set_halign(Align::Center);
        label_current_login.set_halign(Align::Start);
        label_login.set_halign(Align::Start);

        box_current_login.pack_start(&label_current_login, false, false, 5);
        box_current_login.pack_start(&label_login, true, true, 5);

        let input_login = Entry::new();

        container.pack_start(&box_current_login, true, false, 0);
        container.pack_start(&Separator::new(Orientation::Horizontal), false, false, 0);
        container.pack_start(&input_login, true, false, 0);

        Content {
            container,
            input: input_login,
            output: label_login,
        }
    }
}

fn main() {
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK application");
        std::process::exit(1);
    }

    let data = Rc::new(RefCell::new(Data::new()));

    let app = App::new(&data.borrow());

    {
        let data_clone = data.clone();
        let label_login = app.content.output.clone();

        app.header.cancel.clone().connect_clicked(move |_| {
            data_clone.borrow_mut().auth.sign_out();
            label_login.set_label("You are signed out");
        });
    }

    {
        let data_clone = data.clone();
        let input_login = app.content.input.clone();
        let label_login = app.content.output.clone();

        app.header.apply.clone().connect_clicked(move |_| {
            let new_login = input_login.get_buffer().get_text();
            let mut new_label = String::new();

            // if sign-in fails get error message
            if let Err(e) = data_clone.borrow_mut().auth.sign_in(&new_login) {
                new_label = e.to_string();
            }

            // if sign-in was ok get user's login
            if new_label.is_empty() {
                new_label = match data_clone.borrow().auth.login() {
                    Some(login) => login.to_owned(),
                    None => "???? This should not be possible".to_string(),
                };
            }

            // set label with new value
            label_login.set_label(&new_label);
        });
    }

    app.window.show_all();

    gtk::main();
}
