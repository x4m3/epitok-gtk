mod about;
mod app;
mod content;
mod header;
mod settings;
mod storage;
mod strings;
mod ui;

use crate::app::App;
use crate::strings::{PROGRAM_NAME, VERSION};

fn main() {
    println!("{} - {}", PROGRAM_NAME, VERSION);
    App::new()
        .connect_events()
        .try_get_events()
        .start()
        .save_settings();
    println!("Bye bye");
}
