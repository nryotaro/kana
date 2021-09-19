use crate::persistence::samba::SambaClient;
use crate::port::DocumentPort;
use std::sync::mpsc;
use std::thread;

pub enum DocumentMessage {
	ReadRoot {
		uri: String,
		destination: mpsc::Sender<Result<(), String>>,
	},
}
pub fn initialize_document_thread<'a>(
	create_reader: &'static (dyn Fn(&String) -> Box<dyn DocumentPort> + Sync),
) -> mpsc::Sender<DocumentMessage> {
	let (sender, receiver): (
		mpsc::Sender<DocumentMessage>,
		mpsc::Receiver<DocumentMessage>,
	) = mpsc::channel();
	thread::spawn(move || {
		let mut client: Option<Box<dyn DocumentPort>> = None;
		loop {
			let message = &receiver.recv().unwrap();
			match message {
				DocumentMessage::ReadRoot { uri, destination } => {
					client = Some(create_reader(uri));
				}
			}
		}
	});
	sender
}

pub fn create_document_port(url: &String) -> Box<dyn DocumentPort> {
	let l = url.clone().as_str();
	let client: Box<SambaClient> = DocumentPort::new(url.as_str()).unwrap();
	client
}
