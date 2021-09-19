use crate::core::state::Configuration;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

#[derive(Serialize, Deserialize)]
struct ConfigurationFile {
	pub root_uri: Option<String>,
}

pub fn initialize_home(base_dir: &String) -> Result<(), String> {
	fs::create_dir_all(&base_dir).map_err(|_| "Failed to create the base directory.")?;
	let config_file = get_config_file_path(&base_dir);
	if !Path::new(&config_file).exists() {
		let text = serde_json::to_string(&ConfigurationFile { root_uri: None }).unwrap();
		let mut f = fs::File::create(&config_file).unwrap();
		f.write_all(&text.as_ref()).unwrap();
	};
	Ok(())
}

/**
 * base
 */
pub fn get_base_dir() -> String {
	env::var("KANA_HOME").unwrap_or(format!("{}/.kana", env::var("HOME").unwrap()))
}

fn get_config_file_path(base_dir: &String) -> String {
	format!("{}/settings.json", base_dir)
}

pub fn load_config(base_dir: &String) -> Result<Configuration, String> {
	let config_path = get_config_file_path(&base_dir);
	let config_text = fs::read_to_string(config_path);
	let config_file: ConfigurationFile = match config_text {
		Ok(text) => serde_json::from_str(text.as_ref()).unwrap(),
		Err(_) => ConfigurationFile { root_uri: None },
	};
	Ok(Configuration::new(config_file.root_uri))
}

pub fn save_config(configuration: &Configuration) -> Result<(), String> {
	let text = serde_json::to_string(&ConfigurationFile {
		root_uri: configuration.get_root_uri(),
	})
	.unwrap();

	let mut f = OpenOptions::new()
		.append(true)
		.create(true) // Optionally create the file if it doesn't already exist
		.open(get_config_file_path(&get_base_dir()))
		.expect("Unable to open file");
	f.write_all(text.as_ref()).expect("Unable to write data");
	Ok(())
}
