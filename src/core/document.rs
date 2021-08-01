use std::sync::mpsc;

pub enum DocumentMessage {
	ReadRoot {
		uri: String,
		destination: mpsc::Sender<bool>,
	},
}
