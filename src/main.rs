use gtk::prelude::*;
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

    let (sender2, receiver2): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
    let (sender, receiver): (
        mpsc::Sender<(mpsc::Sender<A>, A)>,
        mpsc::Receiver<(mpsc::Sender<A>, A)>,
    ) = mpsc::channel();
    let document_actor = thread::spawn(move || loop {
        let item = receiver.recv().unwrap();
    });

    let configuration = config::load_config();
    let application = ui::initialize();
    application.run();
}

enum A {
    Doge(String),
}
