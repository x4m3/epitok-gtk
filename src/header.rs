use crate::app::App;
use epitok::event::list_events_today;
use gtk::*;

pub struct Header {
    pub container: HeaderBar,
    pub refresh: Button,
    pub spinner: Spinner,
}

impl Header {
    pub fn new() -> Self {
        let container = HeaderBar::new();

        container.set_subtitle("philippe1.loctaux@epitech.eu".into()); // TODO: replace this by real login
        container.set_show_close_button(true);

        let refresh = Button::from_icon_name("view-refresh-symbolic".into(), IconSize::Button);
        let spinner = Spinner::new();

        container.pack_start(&refresh);
        container.pack_start(&spinner);

        Self {
            container,
            refresh,
            spinner,
        }
    }
}

impl App {
    pub fn connect_refresh_event(&self) {
        let ui = self.ui.clone();
        let auth = self.auth.clone();
        let events = self.events.clone();

        self.ui.header.refresh.connect_clicked(move |_| {
            ui.header.refresh.set_sensitive(false);
            ui.header.spinner.start();

            if let Ok(auth) = auth.try_borrow() {
                if let Ok(mut events) = events.try_borrow_mut() {
                    match list_events_today(&mut events, auth.autologin()) {
                        Ok(x) => println!("success: got {} events", x),
                        Err(e) => eprintln!("error: {}", e),
                    }
                }
            }

            ui.header.spinner.stop();
            ui.header.refresh.set_sensitive(true);
        });
    }
}
