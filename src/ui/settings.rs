use crate::core::document;
use gtk::prelude::*;
use gtk::{gdk, gio, SearchEntry};
use std::sync::mpsc;
mod search;

pub fn initialize(
	builder: gtk::Builder,
	document_sender: &mpsc::Sender<document::DocumentMessage>,
) {
	let search_entry: SearchEntry = builder
		.object("root_directory_search")
		.expect("Couldn't find field to search entry.");
	search::initialize_root_search(search_entry, &document_sender);
}
