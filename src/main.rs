use gtk::prelude::*;
use kana::core::document;
use kana::core::state;
use kana::persistence::config;
use kana::ui;
use std::sync::mpsc;

fn main() {
    let base_dir: String = config::get_base_dir();
    config::initialize_home(&base_dir);
    let configuration = config::load_config(&base_dir).unwrap();
    state::initialize_setting_thread(configuration, &config::save_config);

    let (setting_root_sender, setting_root_receiver): (mpsc::Sender<bool>, mpsc::Receiver<bool>) =
        mpsc::channel();
    let sender = document::initialize_document_thread(&document::create_document_port);
    //let configuration = config::load_config();
    let application = ui::initialize(sender);
    application.run();
}

