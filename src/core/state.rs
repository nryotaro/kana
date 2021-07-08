pub trait Configuration {
	fn get_root_directory(&self) -> Option<String>;
}