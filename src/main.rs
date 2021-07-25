use gtk::prelude::*;
use kana::port;
use kana::persistence::samba::{SambaClient};



fn main() {

    let a: Option<Box<SambaClient>> = port::DocumentRepository::new("");
    /*
    let configuration = persistence::config::load_config();
    let application = ui::initialize();
    application.run();
    */
}
