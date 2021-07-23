use gtk::prelude::*;
mod core;
mod persistence;
mod ui;

fn main() {

    persistence::samba::temp();
    /*
    let configuration = persistence::config::load_config();
    let application = ui::initialize();
    application.run();
    */
}
