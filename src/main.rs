use gtk::prelude::*;
use kana::core::document;
use kana::persistence::config;
use kana::persistence::samba::SambaClient;
use kana::port;
use kana::ui;
use std::sync::mpsc;
use std::thread;

fn main() {
    let a: Option<Box<SambaClient>> =
        port::DocumentRepository::new("smb://192.168.1.2/share/documents/manga/");

    let b: Box<dyn kana::port::DocumentRepository> = a.unwrap();
    b.close();

    let (sender2, receiver2): (
        mpsc::Sender<bool>,
        mpsc::Receiver<bool>,
    ) = mpsc::channel();

    let (sender, receiver): (
        mpsc::Sender<document::DocumentMessage>,
        mpsc::Receiver<document::DocumentMessage>,
    ) = mpsc::channel();

    sender.send(document::DocumentMessage::ReadRoot {
        uri: "".to_string(),
        destination: sender2,
    });
    //sender.send();
    let document_actor = thread::spawn(move || loop {
        let (sender, item) = receiver.recv().unwrap();
    });

    let configuration = config::load_config();
    let application = ui::initialize();
    application.run();
}
