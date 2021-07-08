use crate::core::state::{Configuration};

pub struct FileConfig {

}

impl Configuration for FileConfig {
	fn get_root_directory(&self) -> Option<String> {
		Some("".to_string())
	}
}

pub fn load_config() -> impl Configuration  {
	FileConfig{}
}