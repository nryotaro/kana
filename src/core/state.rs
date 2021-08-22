use std::sync::mpsc;
use std::thread;

pub struct Configuration {
	root_uri: Option<String>,
}

impl Configuration {
	pub fn new(root_uri: Option<String>) -> Configuration {
		Configuration { root_uri }
	}

	pub fn get_root_uri(&self) -> Option<String> {
		self.root_uri.clone()
	}
}

pub enum ConfigurationMessage {
	Save {
		configuration: Configuration,
		destination: mpsc::Sender<Result<(), String>>,
	},
	Load {
		destination: mpsc::Sender<Configuration>,
	},
}

pub fn initialize_setting_thread(
	configuration: Configuration,
	save_fn: &'static (dyn Fn(&Configuration) -> Result<(), String> + Sync),
) -> mpsc::Sender<ConfigurationMessage> {
	let (sender, receiver): (
		mpsc::Sender<ConfigurationMessage>,
		mpsc::Receiver<ConfigurationMessage>,
	) = mpsc::channel();

	thread::spawn(move || {
		let mut config: Configuration = configuration;
		loop {
			let message = receiver.recv().unwrap();
			match message {
				ConfigurationMessage::Save {
					configuration,
					destination,
				} => {
					save_fn(&configuration).unwrap();
					destination.send(Ok(())).unwrap();
					config = configuration;
				}
				ConfigurationMessage::Load { destination } => {
					//let a= config.root_uri.clone();
					destination
						.send(Configuration {
							root_uri: config.root_uri.clone(),
						})
						.unwrap();
				}
			};
		}
	});
	sender
}
