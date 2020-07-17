mod about;
mod app;
mod content;
mod header;
mod settings;
mod strings;
mod ui;

use crate::app::App;

fn main() {
    App::new().connect_events().start()
}
