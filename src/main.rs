use gtk::prelude::*;
use kana::persistence::samba::SambaClient;
use kana::port;

fn main() {
    let a: Option<Box<SambaClient>> =
        port::DocumentRepository::new("smb://192.168.1.2/share/documents/manga/");

    let b: Box<dyn kana::port::DocumentRepository> = a.unwrap();
    b.close();
    /*
    let configuration = persistence::config::load_config();
    let application = ui::initialize();
    application.run();
    */
}
