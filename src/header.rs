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

        container.set_subtitle("first.last@epitech.eu".into()); // TODO: replace this by real login
        container.set_show_close_button(true);

        let refresh = Button::from_icon_name("view-refresh-symbolic".into(), IconSize::Button);
        let spinner = Spinner::new();

        let menu_button = MenuButton::new();
        let menu_button_icon = Image::from_icon_name("open-menu-symbolic".into(), IconSize::Button);
        menu_button.add(&menu_button_icon);

        let menu = gio::Menu::new();
        let preferences_button = gio::MenuItem::new("Preferences".into(), None);
        menu.append_item(&preferences_button);
        let about_button = gio::MenuItem::new("About".into(), None);
        menu.append_item(&about_button);
        menu_button.set_menu_model(Some(&menu));

        container.pack_start(&refresh);
        container.pack_start(&spinner);
        container.pack_end(&menu_button);

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

            // TODO: make epitok lib async to actually see the spinner when it's refreshing
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
