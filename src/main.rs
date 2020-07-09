use gtk::*;
// use std::sync::atomic::{AtomicUsize, Ordering};
// use std::sync::Arc;
use std::{cell::RefCell, rc::Rc};

use epitok::auth::Auth;

const PROGRAM_NAME: &str = "epitok";
// const MESSAGES: [&str; 3] = ["hit", "dead", "heal"];

// #[repr(usize)]
// enum Message {
//     Hit = 0,
//     Dead = 1,
//     Heal = 2,
// }

// pub struct HealthComponent(AtomicUsize);

// impl HealthComponent {
//     fn new(initial: usize) -> HealthComponent {
//         HealthComponent(AtomicUsize::new(initial))
//     }

//     fn get_health(&self) -> usize {
//         self.0.load(Ordering::SeqCst)
//     }

//     fn subtract(&self, value: usize) -> usize {
//         let current = self.0.load(Ordering::SeqCst);
//         let new = if current < value { 0 } else { current - value };
//         self.0.store(new, Ordering::SeqCst);
//         new
//     }

//     fn heal(&self, value: usize) -> usize {
//         let orig = self.0.fetch_add(value, Ordering::SeqCst);
//         orig + value
//     }
// }

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

        // let content_login = match data.auth {
        //     Some(auth) => Some(auth.login()),
        //     None => None,
        // };
        // let label_login = Label::new(content_login);
        // let tmp = data.auth.login().to_owned().as_deref();
        let label_login = Label::new(data.auth.login().to_owned().as_deref());

        // let content_current_login = Label::new(match data.auth => )
        // let content_current_login = Label::new(Some(.get_health().to_string().as_str()));

        box_current_login.set_halign(Align::Center);
        label_current_login.set_halign(Align::Start);
        label_login.set_halign(Align::Start);

        // health_info.set_halign(Align::Center);
        // health_label.set_halign(Align::Start);
        // health.set_halign(Align::Start);

        box_current_login.pack_start(&label_current_login, false, false, 5);
        box_current_login.pack_start(&label_login, true, true, 5);

        // health_info.pack_start(&health_label, false, false, 5);
        // health_info.pack_start(&health, true, true, 5);

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

    // let data = Arc::new(Data::new());
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
            // TODO: check if input is not empty
            let new_login = input_login.get_buffer().get_text();

            let mut new_label = String::new();
            println!("starting to sign-in");

            // if sign-in fails get error message
            if let Err(e) = data_clone.borrow_mut().auth.sign_in(&new_login) {
                new_label = e.to_string();
            }
            println!("finished to sign-in");

            // if sign-in was ok get user's login
            if new_label.is_empty() {
                new_label = match data_clone.borrow().auth.login() {
                    Some(login) => login.to_owned(),
                    None => "????".to_string(),
                };
            }
            // let aa_label = match data_clone.borrow_mut().auth.sign_in(&new_login) {
            //     Err(e) => e.to_string(),
            //     Ok(_) => {}
            // };

            // let new_label = match data_clone.borrow_mut().auth.sign_in(&new_login) {
            //     Ok(()) => match data_clone.borrow().auth.login().as_ref() {
            //         Some(a) => a,
            //         _ => unreachable!(),
            //     }
            //     .into(),
            //     Err(e) => e.to_string(),
            // };
            label_login.set_label(&new_label);
        });
    }

    // {
    //     let health = health.clone();
    //     let message = app.content.message.clone();
    //     let info = app.content.health.clone();
    //     app.header.hit.clone().connect_clicked(move |_| {
    //         let new_health = health.subtract(1);
    //         let action = if new_health == 0 {
    //             Message::Dead
    //         } else {
    //             Message::Hit
    //         };
    //         message.set_label(MESSAGES[action as usize]);
    //         info.set_label(new_health.to_string().as_str());
    //     });
    // }

    // {
    //     let health = health.clone();
    //     let message = app.content.message.clone();
    //     let info = app.content.health.clone();
    //     app.header.heal.clone().connect_clicked(move |_| {
    //         let new_health = health.heal(5);
    //         message.set_label(MESSAGES[Message::Heal as usize]);
    //         info.set_label(new_health.to_string().as_str());
    //     });
    // }

    app.window.show_all();

    gtk::main();
}
