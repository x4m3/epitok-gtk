const PROGRAM_NAME: &str = "epitok";

fn main() {
    glib::set_program_name(PROGRAM_NAME.into());
    glib::set_application_name(PROGRAM_NAME);
}
