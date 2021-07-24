use gtk::prelude::*;
mod core;
mod persistence;
mod port;
mod ui;

fn main() {

    let a: persistence::samba::SambaClient = port::DocumentRepository::new("");
    /*
    let configuration = persistence::config::load_config();
    let application = ui::initialize();
    application.run();
    */
}
