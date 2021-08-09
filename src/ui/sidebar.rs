use crate::core::document;
use gtk::gdk_pixbuf::InterpType;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use gtk::{Button, Image, Label, Stack};
use std::io::Cursor;
use std::sync::mpsc;

pub fn initialize(builder: &gtk::Builder) {
	let stack: Stack = builder
		.object("stack")
		.expect("Couldn't find the stack button.");

	let preview_button: Button = builder
		.object("button_preview")
		.expect("Couldn't find the preview button.");

	let preview_stack: Label = builder
		.object("preview_stack")
		.expect("Couldn't find the preview stack.");
	let setting_stack: Label = builder
		.object("setting_stack")
		.expect("Couldn't find the setting stack.");

	let pixbuf = Pixbuf::from_read(Cursor::new(include_bytes!("preview.png"))).unwrap();
	let px = pixbuf.scale_simple(32, 32, InterpType::Nearest);
	let image = Image::from_pixbuf(px.as_ref());
	preview_button.set_image(Some(&image));

	let temp_stack = stack.clone();
	preview_button.connect_clicked(move |_| {
		&stack.set_visible_child(&preview_stack);
	});

	let setting_button: Button = builder
		.object("button_settings")
		.expect("Couldn't find the setting button.");

	let pixbuf2 = Pixbuf::from_read(Cursor::new(include_bytes!("setting.png"))).unwrap();
	let px2 = pixbuf2.scale_simple(32, 32, InterpType::Nearest);
	let image2 = Image::from_pixbuf(px2.as_ref());
	setting_button.set_image(Some(&image2));
	setting_button.connect_clicked(move |_| {
		&temp_stack.set_visible_child(&setting_stack);
		eprintln!("Clicked!");
	});
}
