use epitok::auth::Auth;
use gtk::*;
use std::{cell::RefCell, rc::Rc};

const PROGRAM_NAME: &str = "Epitok";

pub struct App {
    pub ui: Rc<GtkUi>,
    pub auth: Rc<RefCell<Auth>>,
}

pub struct GtkUi {
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
    fn new() -> Self {
        glib::set_program_name(Some(PROGRAM_NAME.into()));

        if gtk::init().is_err() {
            eprintln!("Failed to initialize GTK application");
            std::process::exit(1);
        }

        Self {
            ui: Rc::new(GtkUi::new()),
            auth: Rc::new(RefCell::new(Auth::new())),
        }
    }

    pub fn connect_events(self) -> Self {
        self.connect_cancel();
        self.connect_apply();

        self
    }

    pub fn start(self) {
        self.ui.window.show_all();
        gtk::main();
    }
}

impl GtkUi {
    fn new() -> Self {
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

impl Header {
    fn new() -> Self {
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

impl Content {
    fn new() -> Self {
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

impl App {
    pub fn connect_cancel(&self) {
        let ui = self.ui.clone();
        let auth = self.auth.clone();

        self.ui.header.cancel.connect_clicked(move |_| {
            match auth.try_borrow_mut() {
                Ok(mut auth) => {
                    auth.sign_out();
                    ui.content.output.set_label("You are signed out");
                }
                Err(e) => ui.content.output.set_label(e.to_string().as_str()),
            };
        });
    }

    pub fn connect_apply(&self) {
        let ui = self.ui.clone();
        let auth = self.auth.clone();

        self.ui.header.apply.connect_clicked(move |_| {
            let new_login = ui.content.input.get_buffer().get_text();

            // if sign-in fails get error message
            match auth.try_borrow_mut() {
                Ok(mut auth) => match auth.sign_in(&new_login) {
                    Ok(()) => (),
                    Err(e) => ui.content.output.set_label(e.to_string().as_str()),
                },
                Err(e) => ui.content.output.set_label(e.to_string().as_str()),
            }

            // set label with new value
            match auth.try_borrow() {
                Ok(auth) => match auth.login() {
                    Some(login) => ui.content.output.set_label(login),
                    // None => label_login.set_label("You are not signed in"),
                    _ => (),
                },
                Err(e) => ui.content.output.set_label(e.to_string().as_str()),
            }
        });
    }
}

fn main() {
    App::new().connect_events().start()
    // let auth = Rc::new(RefCell::new(Auth::new()));

    // let app = App::new(&auth.borrow());

    // {
    //     let auth = app.auth.clone();
    //     let label_login = app.ui.content.output.clone();

    //     app.ui.header.cancel.connect_clicked(move |_| {
    //         match auth.try_borrow_mut() {
    //             Ok(mut auth) => {
    //                 auth.sign_out();
    //                 label_login.set_label("You are signed out");
    //             }
    //             Err(e) => label_login.set_label(e.to_string().as_str()),
    //         };
    //     });
    // }

    // {
    //     let input_login = app.ui.content.input.clone();
    //     let label_login = app.ui.content.output.clone();

    //     app.ui.header.apply.connect_clicked(move |_| {
    //         // app.ui.header.apply.clone().connect_clicked(move |_| {
    //         // get contents of input field
    //         let new_login = input_login.get_buffer().get_text();

    //         // if sign-in fails get error message
    //         match app.auth.try_borrow_mut() {
    //             Ok(mut auth) => match auth.sign_in(&new_login) {
    //                 Ok(()) => (),
    //                 Err(e) => label_login.set_label(e.to_string().as_str()),
    //             },
    //             Err(e) => label_login.set_label(e.to_string().as_str()),
    //         }

    //         // set label with new value
    //         match app.auth.try_borrow() {
    //             Ok(auth) => match auth.login() {
    //                 Some(login) => label_login.set_label(login),
    //                 // None => label_login.set_label("You are not signed in"),
    //                 _ => (),
    //             },
    //             Err(e) => label_login.set_label(e.to_string().as_str()),
    //         }
    //     });
    // }
}
