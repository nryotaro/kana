use std::sync::mpsc;
use std::thread;

pub enum DocumentMessage {
	ReadRoot {
		uri: String,
		destination: mpsc::Sender<Result<(), String>>,
	},
}

pub fn initialize_document_thread() -> mpsc::Sender<DocumentMessage> {
	let (sender, receiver): (
		mpsc::Sender<DocumentMessage>,
		mpsc::Receiver<DocumentMessage>,
	) = mpsc::channel();
	thread::spawn(move || loop {
		receiver.recv().unwrap();
	});
	sender
}
