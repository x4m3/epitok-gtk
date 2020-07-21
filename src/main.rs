mod about;
mod app;
mod content;
mod header;
mod settings;
mod storage;
mod strings;
mod ui;

use crate::app::App;

fn main() {
    App::new()
        .load_settings()
        .connect_events()
        .start()
        .save_settings();
}
