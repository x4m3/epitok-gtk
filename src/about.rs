use crate::app::App;
use crate::strings::*;
use gtk::*;

impl App {
    pub fn connect_show_about(&self) {
        let ui = self.ui.clone();

        self.ui.header.about.connect_clicked(move |_| {
            // define about window
            let about = AboutDialog::new();
            about.set_program_name(PROGRAM_NAME.into());
            about.set_version(VERSION.into());
            about.set_logo_icon_name(ICON.into());
            about.set_copyright("© 2020 Philippe Loctaux".into());
            about.set_website("https://github.com/x4m3/epitok-gtk".into());
            about.set_website_label("Source code".into());
            about.set_modal(true);

            // credits
            let authors = &["Philippe Loctaux"];
            about.set_authors(authors);

            let artists = &["Ghassane Sebaï"];
            about.set_artists(artists);

            // set parent window and show about window
            about.set_transient_for(Some(&ui.window));
            about.connect_response(|dialog, _| dialog.close());
            about.show();
        });
    }
}
