use gtk::*;

pub struct Content {
    pub container: Box,
}

impl Content {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 0);

        Self { container }
    }
}
