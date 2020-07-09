use epitok::auth::Auth;
use gtk::*;
use std::{cell::RefCell, rc::Rc};

const PROGRAM_NAME: &str = "epitok";

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

impl App {
    fn new(auth: &Auth) -> App {
        let window = Window::new(gtk::WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new(auth);

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
    fn new(auth: &Auth) -> Content {
        let container = Box::new(Orientation::Vertical, 0);

        let box_current_login = Box::new(Orientation::Horizontal, 0);
        let label_current_login = Label::new("Current login: ".into());

        let label_login = Label::new(auth.login().to_owned().as_deref());

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

    let auth = Rc::new(RefCell::new(Auth::new()));

    let app = App::new(&auth.borrow());

    {
        let auth = auth.clone();
        let label_login = app.content.output.clone();

        app.header.cancel.clone().connect_clicked(move |_| {
            // TODO: borrow_mut -> try_borrow_mut. panic otherwise
            auth.borrow_mut().sign_out();
            label_login.set_label("You are signed out");
        });
    }

    {
        let auth = auth.clone();
        let input_login = app.content.input.clone();
        let label_login = app.content.output.clone();

        app.header.apply.clone().connect_clicked(move |_| {
            let new_login = input_login.get_buffer().get_text();
            let mut new_label = String::new();

            // if sign-in fails get error message
            if let Err(e) = auth.borrow_mut().sign_in(&new_login) {
                new_label = e.to_string();
            }

            // if sign-in was ok get user's login
            if new_label.is_empty() {
                new_label = match auth.borrow().login() {
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
