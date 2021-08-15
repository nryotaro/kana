use crate::core::state::Configuration;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
/*
pub struct FileConfig {}

impl Configuration for FileConfig {
	fn get_root_directory(&self) -> Option<String> {
		Some("".to_string())
	}
}
pub fn load_config(config_path: String) -> Configuration {
	//FileConfig {}
}

use std::fs;

fn main() -> std::io::Result<()> {
	fs::create_dir("/some/dir")?;
	Ok(())
}
Run
*/

#[derive(Serialize, Deserialize)]
struct Person {
	name: String,
	age: u8,
	phones: Vec<String>,
}

pub fn initialize_base_dir(base_dir: String) -> Result<(), String> {
	match fs::create_dir_all(base_dir) {
		Ok(_) => Ok(()),
		Err(_) => Err(String::from("Failed to create the base directory.")),
	}
}

pub fn get_base_dir() -> String {
	env::var("KANA_HOME").unwrap_or(format!("{}/.kana", env::var("HOME").unwrap()))
}

pub fn load_config(base_dir: String) -> Configuration {
	let config_file = format!("{}/settings.json", base_dir);
	match fs::read_to_string(config_file) {
		Ok(text) => Configuration::new(),
		Err(err) => Configuration::new(None),
	}
}
