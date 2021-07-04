use gtk::prelude::*;
use gtk::{gdk, gio, SearchEntry};
mod search;

pub fn initialize(builder: gtk::Builder) {
	let search_entry: SearchEntry = builder
		.object("root_directory_search")
		.expect("Couldn't find field to search entry.");
	search::initialize_root_search(search_entry);
}
