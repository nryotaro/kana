use crate::core::document;
use gtk::gdk_pixbuf::{Colorspace, InterpType};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::glib::Bytes;
use gtk::prelude::*;
use gtk::Button;
use gtk::Image;
use std::fs::File;
use std::io::Cursor;
use std::sync::mpsc;

pub fn initialize(builder: &gtk::Builder) {
	let preview_button: Button = builder
		.object("button_preview")
		.expect("Couldn't find field to search entry.");
	let temp = include_bytes!("preview.png");
	
	let pixbuf = Pixbuf::from_read(Cursor::new(temp)).unwrap();
	let px = pixbuf.scale_simple(32, 32, InterpType::Nearest);
	let image = Image::from_pixbuf(px.as_ref());
	preview_button.set_image(Some(&image));
	preview_button.set_always_show_image(true);

	/*
	search::initialize_root_search(search_entry, &document_sender);
	*/
}
