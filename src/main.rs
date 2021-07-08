use gtk::prelude::*;
mod core;
mod ui;
mod persistence;

fn main() {
    let configuration = persistence::config::load_config();
    let application = ui::initialize();
    application.run();
}
