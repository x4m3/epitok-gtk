mod app;
mod content;
mod header;
mod strings;
mod ui;

use crate::app::App;

fn main() {
    App::new().connect_events().start()
}
